use crate::infrastructures::scraper;
use crate::models::{EbookSnapshot, Payment};
use anyhow::Result;
use headless_chrome::Browser;
use once_cell::sync::Lazy;
use pure_funcs::get_now_in_sec;
use regex::Regex;
use std::collections::HashSet;
use url::Url;

static PRICE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"￥(?P<price>\d{1,3}(,\d{3})*)").unwrap());
static POINT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<points>\d{1,3}(,\d{3})*)pt").unwrap());

pub fn create_url(id: &str) -> Result<Url> {
    let url = Url::parse("https://www.amazon.co.jp/dp/")?;
    let joined = url.join(id)?;
    Ok(joined)
}

fn extract_price(text: &str) -> HashSet<&str> {
    let matched = PRICE_REGEX
        .captures_iter(text)
        .map(|cap| cap.name("price").unwrap().as_str())
        .collect::<Vec<_>>();
    matched.into_iter().collect()
}

fn extract_points(text: &str) -> HashSet<&str> {
    let matched = POINT_REGEX
        .captures_iter(text)
        .map(|cap| cap.name("points").unwrap().as_str())
        .collect::<Vec<_>>();
    matched.into_iter().collect()
}

fn extract_payment(text: &str) -> Option<Payment> {
    let price = *extract_price(text).iter().next()?;
    let points = *extract_points(text).iter().next()?;
    Some(Payment {
        price: price.to_string(),
        points: points.to_string(),
    })
}

pub fn get(id: &str) -> Result<EbookSnapshot> {
    let url = create_url(id)?;
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(url.as_str())?;
    tab.wait_for_element("#navFooter")?;

    let title_element = tab.find_element("#productTitle")?;
    let title = title_element.get_inner_text()?;

    let image = tab.find_element("#ebooksImgBlkFront")?;
    let image_attribute = image.get_attributes()?.unwrap();
    let thumbnail_url_str = scraper::search_from(&image_attribute, "src").unwrap();
    let thumbnail_url = Url::parse(thumbnail_url_str.as_str())?;

    let price_list = tab.find_elements("#tmmSwatches .a-button-text")?;
    let payments = price_list
        .iter()
        .flat_map(|price| price.get_inner_text())
        .flat_map(|text| {
            let payment = extract_payment(text.as_str());
            payment
        })
        .collect::<Vec<_>>();

    let snapshot = EbookSnapshot {
        ebook_id: id.to_string(),
        scraped_at: get_now_in_sec(),
        title: String::from(title.trim()),
        thumbnail_url,
        payment_ebook: payments.get(0).map(|x| x.clone()),
        payment_real: payments.get(1).map(|x| x.clone()),
    };

    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_get() {
        let id = String::from("B09RQGMYKZ");
        let actual = get(id.as_str()).unwrap();
        assert_eq!(actual.ebook_id, id);
        assert_eq!(actual.title, String::from("コンセプトから理解するRust"));
        assert_eq!(
            actual.thumbnail_url.to_string(),
            String::from("https://m.media-amazon.com/images/I/51CTnaTcJtL.jpg")
        );
        assert_debug_snapshot!(actual.payment_ebook);
        assert_debug_snapshot!(actual.payment_real);
    }

    #[test]
    fn test_extract_payment() {
        let actual = extract_payment(
            r#"
            Kindle版 (電子書籍)
            ￥3,344
            獲得ポイント: 152pt
            "#,
        );
        let expected = Some(Payment {
            price: String::from("3,344"),
            points: String::from("152"),
        });
        assert_eq!(actual, expected);
    }
}
