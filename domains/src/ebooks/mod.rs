mod model;
mod repositories;

use crate::ebook_snapshots;
use crate::infrastructures::prisma;
use crate::infrastructures::prisma::PrismaClient;
use crate::infrastructures::scraper;
use anyhow::Result;
use futures::stream;
use futures::{future, StreamExt};
use headless_chrome::Browser;

pub async fn snap_ebook(client: &PrismaClient, browser: &Browser, id: String) -> Result<()> {
    let snapshot = scraper::ebook_snapshot::get(browser, id.as_str())?;
    let result = ebook_snapshots::repositories::db::insert(client, &snapshot).await?;
    Ok(result)
}

pub async fn snap_all_ebook() -> Result<()> {
    let client = prisma::new_client().await?;
    let lists = repositories::db::select_all(&client).await?;
    let browser = Browser::default()?;

    let futures = lists
        .into_iter()
        .map(|ebook| snap_ebook(&client, &browser, ebook.id))
        .collect::<Vec<_>>();
    let stream = stream::iter(futures).buffer_unordered(3);
    let results = stream.collect::<Vec<_>>().await;
    Ok(())
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

    // #[tokio::test] // 必要な時だけ動かす
    async fn it_works_snap_all_ebook() {
        dotenv::dotenv().ok();

        let actual = snap_all_ebook().await.unwrap();
        assert_eq!(actual, ());
    }
}
