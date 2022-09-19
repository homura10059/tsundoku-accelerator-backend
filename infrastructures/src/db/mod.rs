pub mod prisma;

use crate::db::prisma::ebook::Data;
use anyhow::Result;
use prisma::PrismaClient;
use prisma::{ebook, ebook_in_wish_list, wish_list};
use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct ItemMetaData {
    id: String,
    url: Url,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WishListSnapshot {
    id: String,
    title: String,
    url: Url,
    scraped_at: i64,
    items: Vec<ItemMetaData>,
}

async fn upsert_items(client: &PrismaClient, items: &Vec<ItemMetaData>) -> Result<Vec<Data>> {
    let upsert_target = items.to_vec();
    let item_upsert: Vec<_> = upsert_target
        .into_iter()
        .map(|item| {
            client.ebook().upsert(
                ebook::id::equals(item.id.clone()),
                ebook::create(item.id.clone(), item.url.to_string(), vec![]),
                vec![],
            )
        })
        .collect();
    let items: Vec<_> = client._batch(item_upsert).await?;
    Ok(items)
}

async fn upsert_wish_list(client: &PrismaClient, snapshot: &WishListSnapshot) -> Result<()> {
    let items = upsert_items(client, &snapshot.items).await?;

    let wish_list = client
        .wish_list()
        .upsert(
            wish_list::id::equals(snapshot.id.clone()),
            wish_list::create(
                snapshot.id.clone(),
                snapshot.url.clone().to_string(),
                snapshot.scraped_at.clone(),
                snapshot.title.clone(),
                vec![],
            ),
            vec![
                wish_list::SetParam::SetScrapedAt(snapshot.scraped_at.clone()),
                wish_list::SetParam::SetTitle(snapshot.title.clone()),
            ],
        )
        .exec()
        .await?;

    // item と wish_list の relation を洗い替えする
    client
        .ebook_in_wish_list()
        .delete_many(vec![ebook_in_wish_list::WhereParam::WishListIdEquals(
            wish_list.id.clone(),
        )])
        .exec()
        .await?;
    let connect: Vec<_> = items
        .into_iter()
        .map(|item| (wish_list.id.clone(), item.id.clone(), vec![]))
        .collect();
    client
        .ebook_in_wish_list()
        .create_many(connect)
        .exec()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use pure_funcs::get_now_in_sec;

    async fn client_helper() -> PrismaClient {
        let client = match prisma::new_client().await {
            Ok(c) => c,
            Err(error) => {
                panic!("Failed to open prisma client : {:?}", error)
            }
        };
        client
    }

    fn items_helper() -> Vec<ItemMetaData> {
        let items = vec![
            ItemMetaData {
                id: String::from("B09RQGMYKZ"),
                url: Url::parse("https://www.amazon.co.jp/dp/B09RQGMYKZ").unwrap(),
            },
            ItemMetaData {
                id: String::from("B09WQT2DQD"),
                url: Url::parse("https://www.amazon.co.jp/dp/B09WQT2DQD").unwrap(),
            },
        ];
        items
    }

    #[tokio::test]
    async fn test_upsert_items() {
        dotenv::dotenv().ok();

        let client = client_helper().await;
        let items = items_helper();

        let actual = upsert_items(&client, &items.to_vec()).await.unwrap();
        assert_eq!(actual.len(), 2)
    }

    #[tokio::test]
    async fn test_upsert_wish_list() {
        dotenv::dotenv().ok();

        let client = client_helper().await;
        let items = items_helper();
        let expected = WishListSnapshot {
            id: String::from("2BDAPI9RQ09E9"),
            url: Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap(),
            title: String::from("test_title"),
            scraped_at: get_now_in_sec(),
            items,
        };

        let actual = upsert_wish_list(&client, &expected).await.unwrap();
        assert_eq!(actual, ())
    }
}
