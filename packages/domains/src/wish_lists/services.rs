use db_client::prisma;
use db_client::prisma::ebook_snapshot::OrderByParam;
use db_client::prisma::wish_list::Data as WishListData;
use db_client::prisma::{ebook, ebook_in_wish_list, wish_list, PrismaClient};

use crate::wish_lists::repositories;
use anyhow::Result;
use futures::stream;
use futures::StreamExt;
use headless_chrome::Browser;
use prisma_client_rust::Direction::Desc;

pub async fn select_all_wish_list_and_snapshot() -> Result<Vec<WishListData>> {
    let client = prisma::new_client().await?;
    let wish_lists = client
        .wish_list()
        .find_many(vec![])
        .with(
            wish_list::ebook_in_wish_list::fetch(vec![]).with(
                ebook_in_wish_list::ebook::fetch()
                    .with(ebook::snapshots::fetch(vec![]).order_by(OrderByParam::ScrapedAt(Desc))),
            ),
        )
        .exec()
        .await?;
    Ok(wish_lists)
}

pub async fn update_wish_list(client: &PrismaClient, browser: &Browser, id: String) -> Result<()> {
    let snapshot = repositories::get_wish_list_snapshot(browser, id.as_str())?;
    repositories::upsert_wish_list(client, &snapshot).await?;
    Ok(())
}

pub async fn update_all_wish_list() -> Result<()> {
    let client = prisma::new_client().await?;
    let browser = Browser::default()?;
    let lists = repositories::select_all_wish_list(&client).await?;

    let futures = lists
        .into_iter()
        .map(|list| update_wish_list(&client, &browser, list.id))
        .collect::<Vec<_>>();
    let stream = stream::iter(futures).buffer_unordered(3);
    stream.collect::<Vec<_>>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[tokio::test]
    async fn it_works_update_wish_list() {
        dotenv::dotenv().ok();
        let client = prisma::new_client().await.unwrap();
        let browser = Browser::default().unwrap();

        update_wish_list(&client, &browser, String::from("2BDAPI9RQ09E9"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn it_works_update_all_wish_list() {
        dotenv::dotenv().ok();

        update_all_wish_list().await.unwrap();
    }
}
