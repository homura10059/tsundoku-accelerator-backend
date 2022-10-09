use crate::infrastructures::db::prisma::{ebook, ebook_snapshot, PrismaClient};
use crate::models::EbookSnapshot;
use anyhow::{anyhow, Result};
use math::round;

pub async fn insert(client: &PrismaClient, ebook_snapshot: EbookSnapshot) -> Result<()> {
    let payment_ebook = ebook_snapshot.payment_ebook.ok_or(anyhow!(
        "missing payment_ebook in id:{}",
        ebook_snapshot.ebook_id
    ))?;

    let price = payment_ebook.price.parse::<f64>()?;
    let points = payment_ebook.points.parse::<f64>()?;

    let points_rate = round::floor(points / price, 2);

    let payment_real = ebook_snapshot.payment_real;

    let real_price = payment_real
        .map(|payment| payment.price.parse::<f64>().ok())
        .flatten();
    let discount = real_price.map(|real| real - price);
    let discount_rate = discount.map(|dis| round::floor(dis / price, 2));

    client
        .ebook_snapshot()
        .create(
            ebook::UniqueWhereParam::IdEquals(ebook_snapshot.ebook_id),
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
    use crate::infrastructures::db::get_client;
    use crate::models::Payment;
    use dotenv;
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

        let client = get_client().await.unwrap();

        let actual = insert(&client, expected).await.unwrap();
        assert_eq!(actual, ())
    }
}
