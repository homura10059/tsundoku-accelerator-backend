pub mod repositories;

use crate::infrastructures::prisma;
use crate::infrastructures::prisma::PrismaClient;
use anyhow::Result;
use futures::stream;
use futures::{future, StreamExt};
use headless_chrome::Browser;

pub async fn snap_ebook(client: &PrismaClient, browser: &Browser, id: String) -> Result<()> {
    let snapshot = repositories::scraper::get(browser, id.as_str())?;
    let result = repositories::db::insert(client, &snapshot).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

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
