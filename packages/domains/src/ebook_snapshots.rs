pub mod repositories;

use anyhow::Result;
use headless_chrome::Browser;
use infrastructures::prisma::PrismaClient;
use url::Url;

pub async fn snap_ebook(client: &PrismaClient, browser: &Browser, id: String) -> Result<()> {
    let snapshot = repositories::scraper::get(browser, id.as_str())?;
    repositories::db::insert(client, &snapshot).await?;
    Ok(())
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Payment {
    pub price: String,
    pub points: String,
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

        let actual = snap_ebook(&client, &browser, String::from("B00XV8YCJI"))
            .await
            .unwrap();
        assert_eq!(actual, ());
    }
}
