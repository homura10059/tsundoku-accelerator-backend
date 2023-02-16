pub mod repositories;

use anyhow::Result;
use headless_chrome::Browser;
use infrastructures::prisma::PrismaClient;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use url::Url;

pub async fn snap_ebook(client: &PrismaClient, browser: &Browser, id: String) -> Result<()> {
    let snapshot = repositories::get(browser, id.as_str())?;
    repositories::insert(client, &snapshot).await?;
    Ok(())
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Payment {
    pub price: String,
    pub points: String,
}

static PRICE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"￥(?P<price>\d{1,3}(,\d{3})*)").unwrap());
static POINT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<points>\d{1,3}(,\d{3})*)pt").unwrap());

impl Payment {
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

    pub fn new<T: AsRef<str>>(text: T) -> Option<Self> {
        let price = *Payment::extract_price(text.as_ref()).iter().next()?;
        let points = *Payment::extract_points(text.as_ref()).iter().next()?;
        Some(Payment {
            price: price.to_string().trim().replace(',', ""),
            points: points.to_string().trim().replace(',', ""),
        })
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct EbookSnapshot {
    pub ebook_id: String,
    pub scraped_at: i64,
    pub thumbnail_url: Url,
    pub payment_ebook: Option<Payment>,
    pub payment_real: Option<Payment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use infrastructures::prisma;

    #[tokio::test]
    async fn it_works_snap_ebook() {
        dotenv::dotenv().ok();
        let client = prisma::new_client().await.unwrap();
        let browser = Browser::default().unwrap();

        snap_ebook(&client, &browser, String::from("B00XV8YCJI"))
            .await
            .unwrap();
    }

    #[test]
    fn test_payment_new() {
        let actual = Payment::new(
            r#"
            Kindle版 (電子書籍)
            ￥3,344
            獲得ポイント: 152pt
            "#,
        );
        let expected = Some(Payment {
            price: String::from("3344"),
            points: String::from("152"),
        });
        assert_eq!(actual, expected);
    }
}
