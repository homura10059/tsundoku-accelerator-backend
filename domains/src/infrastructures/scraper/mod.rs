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
    let title = search_from(&a_tag_attributes, "title").unwrap();

    let attributes = elm.get_attributes()?.unwrap();
    let price = search_from(&attributes, "data-price").unwrap();

    let meta = item_meta_data::create(href, title, price)?;
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

    #[test]
    fn test_get_item_urls() {
        let id = String::from("2BDAPI9RQ09E9");
        let url = Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap();
        let actual = get_wish_list_snapshot(id.as_str()).unwrap();
        assert_eq!(actual.id, id);
        assert_eq!(actual.url, url);
        assert_eq!(actual.title, String::from("do_not_delete"));
        insta::assert_debug_snapshot!(actual.items);
    }
}
