use crate::infrastructures::prisma::ebook::Data as EbookData;
use crate::infrastructures::prisma::wish_list::Data as WishListData;
use crate::infrastructures::prisma::{ebook, ebook_in_wish_list, wish_list, PrismaClient};
use crate::item_metadata::ItemMetaData;
use crate::wish_list_snapshot::WishListSnapshot;
use anyhow::Result;

pub async fn upsert_items(
    client: &PrismaClient,
    items: &Vec<ItemMetaData>,
) -> Result<Vec<EbookData>> {
    let upsert_target = items.to_vec();
    let item_upsert: Vec<_> = upsert_target
        .into_iter()
        .map(|item| {
            let price = item.price.parse::<f64>().unwrap();
            client.ebook().upsert(
                ebook::id::equals(item.id.clone()),
                ebook::create(
                    item.id.clone(),
                    item.url.to_string(),
                    item.title,
                    price,
                    vec![],
                ),
                vec![],
            )
        })
        .collect();
    let items: Vec<_> = client._batch(item_upsert).await?;
    Ok(items)
}

pub async fn upsert_wish_list(client: &PrismaClient, snapshot: &WishListSnapshot) -> Result<()> {
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
                wish_list::SetParam::SetUrl(snapshot.url.to_string().clone()),
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

pub async fn select_all_wish_list(client: &PrismaClient) -> Result<Vec<WishListData>> {
    let wish_lists = client
        .wish_list()
        .find_many(vec![])
        .with(
            wish_list::ebook_in_wish_list::fetch(vec![])
                .with(ebook_in_wish_list::ebook::fetch().with(ebook::snapshots::fetch(vec![]))),
        )
        .exec()
        .await?;
    Ok(wish_lists)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructures::prisma;
    use chrono::Utc;
    use dotenv;
    use url::Url;

    fn items_helper() -> Vec<ItemMetaData> {
        let items = vec![
            ItemMetaData {
                id: String::from("B09RQGMYKZ"),
                url: Url::parse("https://www.amazon.co.jp/dp/B09RQGMYKZ").unwrap(),
                title: String::from("title"),
                price: String::from("100.0"),
            },
            ItemMetaData {
                id: String::from("B09WQT2DQD"),
                url: Url::parse("https://www.amazon.co.jp/dp/B09WQT2DQD").unwrap(),
                title: String::from("title"),
                price: String::from("100.0"),
            },
        ];
        items
    }

    #[tokio::test]
    async fn test_upsert_items() {
        dotenv::dotenv().ok();

        let client = prisma::new_client().await.unwrap();
        let items = items_helper();

        let actual = upsert_items(&client, &items.to_vec()).await.unwrap();
        assert_eq!(actual.len(), 2)
    }

    #[tokio::test]
    async fn test_upsert_wish_list() {
        dotenv::dotenv().ok();

        let client = prisma::new_client().await.unwrap();
        let items = items_helper();
        let expected = WishListSnapshot {
            id: String::from("2BDAPI9RQ09E9"),
            url: Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap(),
            title: String::from("test_title"),
            scraped_at: Utc::now().timestamp(),
            items,
        };

        let actual = upsert_wish_list(&client, &expected).await.unwrap();
        assert_eq!(actual, ())
    }

    #[tokio::test]
    async fn test_select_all_wish_list() {
        dotenv::dotenv().ok();

        let client = prisma::new_client().await.unwrap();
        let actual = select_all_wish_list(&client).await.unwrap();

        assert!(actual.len() > 0)
    }
}
