use crate::item_metadata::ItemMetaData;
use crate::wish_list_snapshot::WishListSnapshot;
use chrono::Utc;
use headless_chrome::{Browser, Element};
use infrastructures::prisma::ebook::Data as EbookData;
use infrastructures::prisma::wish_list::Data as WishListData;
use infrastructures::prisma::{ebook, ebook_in_wish_list, wish_list, PrismaClient};
use url::Url;

pub async fn upsert_items(
    client: &PrismaClient,
    items: &[ItemMetaData],
) -> anyhow::Result<Vec<EbookData>> {
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

pub async fn upsert_wish_list(
    client: &PrismaClient,
    snapshot: &WishListSnapshot,
) -> anyhow::Result<()> {
    let items = upsert_items(client, &snapshot.items).await?;

    let wish_list = client
        .wish_list()
        .upsert(
            wish_list::id::equals(snapshot.id.clone()),
            wish_list::create(
                snapshot.id.clone(),
                snapshot.url.clone().to_string(),
                snapshot.scraped_at,
                snapshot.title.clone(),
                vec![],
            ),
            vec![
                wish_list::SetParam::SetUrl(snapshot.url.to_string().clone()),
                wish_list::SetParam::SetScrapedAt(snapshot.scraped_at),
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
        .map(|item| (wish_list.id.clone(), item.id, vec![]))
        .collect();
    client
        .ebook_in_wish_list()
        .create_many(connect)
        .exec()
        .await?;

    Ok(())
}

pub async fn select_all_wish_list(client: &PrismaClient) -> anyhow::Result<Vec<WishListData>> {
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

fn create_url(id: &str) -> anyhow::Result<Url> {
    let url = Url::parse("https://www.amazon.jp/hz/wishlist/ls/")?;
    let joined = url.join(id)?;
    Ok(joined)
}

fn get_item(elm: &Element) -> anyhow::Result<ItemMetaData> {
    let a_tag = elm.find_element(".a-link-normal")?;
    let a_tag_attributes = a_tag.get_attributes()?.unwrap();
    let href = infrastructures::scraper::search_from(&a_tag_attributes, "href").unwrap();
    let title = infrastructures::scraper::search_from(&a_tag_attributes, "title").unwrap();

    let attributes = elm.get_attributes()?.unwrap();
    let price = infrastructures::scraper::search_from(&attributes, "data-price").unwrap();

    let meta = ItemMetaData::new(href, title, price)?;
    Ok(meta)
}

pub fn get_wish_list_snapshot(id: &str) -> anyhow::Result<WishListSnapshot> {
    let url = create_url(id)?;
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;
    tab.navigate_to(url.as_str())?;

    if tab.find_element("#endOfListMarker").is_err() {
        let nav_to_top = tab.wait_for_element("#navBackToTop")?;
        nav_to_top.scroll_into_view()?;
    }

    let selector_string = format!("[data-id=\"{}\"]", &id);
    let selector = selector_string.as_str();
    let item_list = tab.find_elements(selector)?;
    let mut items: Vec<_> = item_list
        .iter()
        .map(get_item)
        .filter_map(|x| x.ok())
        .collect();

    items.sort();
    items.dedup();

    let title_element = tab.find_element("#profile-list-name")?;
    let title = title_element.get_inner_text()?;

    let snapshot = WishListSnapshot {
        id: id.to_string(),
        url,
        scraped_at: Utc::now().timestamp(),
        title,
        items,
    };

    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wish_lists::repositories::{select_all_wish_list, upsert_items, upsert_wish_list};
    use chrono::Utc;
    use dotenv;
    use infrastructures::prisma;
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

        upsert_wish_list(&client, &expected).await.unwrap();
    }

    #[tokio::test]
    async fn test_select_all_wish_list() {
        dotenv::dotenv().ok();

        let client = prisma::new_client().await.unwrap();
        let actual = select_all_wish_list(&client).await.unwrap();

        assert!(!actual.is_empty())
    }

    #[test]
    fn test_create_url() {
        assert_eq!(
            create_url("2BDAPI9RQ09E9").unwrap(),
            Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap()
        );
    }

    #[test]
    fn test_get_wish_list_snapshot() {
        let id = String::from("2BDAPI9RQ09E9");
        let url = Url::parse("https://www.amazon.co.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap();
        let actual = get_wish_list_snapshot(id.as_str()).unwrap();
        assert_eq!(actual.id, id);
        assert_eq!(actual.url, url);
        assert_eq!(actual.title, String::from("do_not_delete"));
        insta::assert_debug_snapshot!(actual.items);
    }
}
