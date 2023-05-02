use crate::domains::ebook_snapshots::{EbookSnapshot, Payment};
use anyhow::anyhow;
use chrono::Utc;
use db_client::prisma::{ebook, ebook_snapshot, PrismaClient};
use headless_chrome::Browser;
use math::round;
use scraper::dict::from;
use url::Url;

pub fn create_url(id: &str) -> anyhow::Result<Url> {
    let url = Url::parse("https://www.amazon.co.jp/dp/")?;
    let joined = url.join(id)?;
    Ok(joined)
}

pub fn get(browser: &Browser, id: &str) -> anyhow::Result<EbookSnapshot> {
    let url = create_url(id)?;

    let tab = browser.new_tab()?;
    tab.navigate_to(url.as_str())?;
    tab.wait_for_element("#navFooter")?;

    let image = tab.find_element("#ebooksImgBlkFront")?;
    let image_attribute = image.get_attributes()?.unwrap();
    let dict = from(&image_attribute);
    let thumbnail_url_str = dict.get("src").unwrap();
    let thumbnail_url = Url::parse(thumbnail_url_str.as_str())?;

    let price_list = tab.find_elements("#tmmSwatches .a-button-text")?;
    let payments = price_list
        .iter()
        .flat_map(|price| price.get_inner_text())
        .flat_map(Payment::new)
        .collect::<Vec<_>>();

    let snapshot = EbookSnapshot {
        ebook_id: id.to_string(),
        scraped_at: Utc::now().timestamp(),
        thumbnail_url,
        payment_ebook: payments.get(0).cloned(),
        payment_real: payments.get(1).cloned(),
    };

    tab.close(true)?;

    Ok(snapshot)
}

pub async fn insert(client: &PrismaClient, ebook_snapshot: &EbookSnapshot) -> anyhow::Result<()> {
    let payment_ebook = ebook_snapshot.payment_ebook.clone().ok_or(anyhow!(
        "missing payment_ebook in id:{}",
        ebook_snapshot.ebook_id
    ))?;

    let price = payment_ebook.price.parse::<f64>()?;
    let points = payment_ebook.points.parse::<f64>()?;

    let points_rate = round::floor(points / price * 100.0, 2);

    let payment_real = ebook_snapshot.payment_real.clone();

    let real_price = payment_real.and_then(|payment| payment.price.parse::<f64>().ok());
    let discount = real_price.map(|real| real - price);
    let discount_rate = discount.map(|dis| round::floor(dis / price * 100.0, 2));

    client
        .ebook_snapshot()
        .create(
            ebook::UniqueWhereParam::IdEquals(ebook_snapshot.ebook_id.clone()),
            ebook_snapshot.scraped_at,
            ebook_snapshot.thumbnail_url.to_string(),
            price,
            points,
            points_rate,
            vec![
                ebook_snapshot::SetParam::SetDiscount(discount),
                ebook_snapshot::SetParam::SetDiscountRate(discount_rate),
            ],
        )
        .exec()
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::ebook_snapshots::{EbookSnapshot, Payment};
    use db_client::prisma;
    use dotenv;
    use insta::assert_debug_snapshot;
    use url::Url;

    #[tokio::test]
    async fn test_insert() {
        dotenv::dotenv().ok();

        let payment_ebook = Payment {
            price: String::from("1000"),
            points: String::from("11"),
        };

        let payment_real = Payment {
            price: String::from("1100"),
            points: String::from("11"),
        };

        let expected = EbookSnapshot {
            ebook_id: String::from("B09RQGMYKZ"),
            scraped_at: 0,
            thumbnail_url: Url::parse("https://m.media-amazon.com/images/I/51CTnaTcJtL.jpg")
                .unwrap(),
            payment_ebook: Some(payment_ebook),
            payment_real: Some(payment_real),
        };

        let client = prisma::new_client().await.unwrap();

        insert(&client, &expected).await.unwrap()
    }

    #[test]
    fn test_get() {
        let browser = Browser::default().unwrap();
        let id = String::from("B09RQGMYKZ");
        let actual = get(&browser, id.as_str()).unwrap();
        assert_eq!(actual.ebook_id, id);
        assert_debug_snapshot!(actual.payment_ebook);
        assert_debug_snapshot!(actual.payment_real);
    }
}
