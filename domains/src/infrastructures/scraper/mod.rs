mod item_meta_data;
mod wish_list;

use crate::models::ItemMetaData;
use crate::models::WishListSnapshot;
use anyhow::Result;
use headless_chrome::{Browser, Element};
use pure_funcs::get_now_in_sec;
use url::Url;

fn search_from(attributes: &Vec<String>, key: &str) -> Option<String> {
    let target = attributes.iter().position(|attr| attr == key)? + 1;
    let (_pos, attr) = attributes
        .iter()
        .enumerate()
        .find(|(pos, _attr)| pos.eq(&target))?;
    let result = attr.clone();
    Some(result)
}

fn get_item(elm: &Element) -> Result<ItemMetaData> {
    let a_tag = elm.find_element(".a-link-normal")?;
    let a_tag_attributes = a_tag.get_attributes()?.unwrap();
    let href = search_from(&a_tag_attributes, "href").unwrap();

    let attributes = elm.get_attributes()?.unwrap();
    let price = search_from(&attributes, "data-price").unwrap();

    let meta = item_meta_data::create(href, price)?;
    Ok(meta)
}

pub fn get_wish_list_snapshot(id: &str) -> Result<WishListSnapshot> {
    let url = wish_list::create_url(id)?;
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(url.as_str())?;

    let nav_to_top = tab.wait_for_element("#navBackToTop")?;

    while tab.find_element("#endOfListMarker").is_err() {
        nav_to_top.scroll_into_view()?;
        break;
    }

    let selector_string = format!("[data-id=\"{}\"]", &id);
    let selector = selector_string.as_str();
    let item_list = tab.find_elements(selector)?;
    let mut items: Vec<_> = item_list
        .iter()
        .map(|item| get_item(item))
        .filter_map(|x| x.ok())
        .collect();

    items.sort();
    items.dedup();

    let title_element = tab.find_element("#profile-list-name")?;
    let title = title_element.get_inner_text()?;

    let snapshot = WishListSnapshot {
        id: id.to_string(),
        url,
        scraped_at: get_now_in_sec(),
        title,
        items,
    };

    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_items_mock() -> Vec<ItemMetaData> {
        let mut items = vec![
            ItemMetaData {
                id: String::from("B08S7CJV4X"),
                url: Url::parse("https://www.amazon.co.jp/dp/B08S7CJV4X/").unwrap(),
                price: String::from("1188.0"),
            },
            ItemMetaData {
                id: String::from("B08L53NT5P"),
                url: Url::parse("https://www.amazon.co.jp/dp/B08L53NT5P/").unwrap(),
                price: String::from("2250.0"),
            },
            ItemMetaData {
                id: String::from("B08L54335M"),
                url: Url::parse("https://www.amazon.co.jp/dp/B08L54335M/").unwrap(),
                price: String::from("3375.0"),
            },
            ItemMetaData {
                id: String::from("B08L51YSLR"),
                url: Url::parse("https://www.amazon.co.jp/dp/B08L51YSLR/").unwrap(),
                price: String::from("2751.0"),
            },
            ItemMetaData {
                id: String::from("B08L5278XF"),
                url: Url::parse("https://www.amazon.co.jp/dp/B08L5278XF/").unwrap(),
                price: String::from("2751.0"),
            },
            ItemMetaData {
                id: String::from("B09TPLQGKS"),
                url: Url::parse("https://www.amazon.co.jp/dp/B09TPLQGKS/").unwrap(),
                price: String::from("396.0"),
            },
            ItemMetaData {
                id: String::from("B0B5Q2RMX6"),
                url: Url::parse("https://www.amazon.co.jp/dp/B0B5Q2RMX6/").unwrap(),
                price: String::from("3168.0"),
            },
        ];
        items.sort();
        items
    }

    #[test]
    fn test_get_item_urls() {
        let items = get_items_mock();
        let expected = WishListSnapshot {
            id: String::from("2BDAPI9RQ09E9"),
            url: Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap(),
            title: String::from("do_not_delete"),
            scraped_at: 0,
            items,
        };
        let actual = get_wish_list_snapshot(expected.id.as_str()).unwrap();
        assert_eq!(actual.id, expected.id);
        assert_eq!(actual.url, expected.url);
        assert_eq!(actual.title, expected.title);
        assert_eq!(actual.items, expected.items);
    }
}
