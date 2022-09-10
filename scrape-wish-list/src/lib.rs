use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

#[derive(Debug, PartialEq)]
pub struct WishListSnapshot {
    id: String,
    title: String,
    url: Url,
    items: Vec<Url>,
}

fn create_item_url(href: String) -> Result<Url> {
    let url = Url::parse("https://www.amazon.co.jp")?;
    let mut joined = url.join(href.as_str())?;
    joined.set_query(None);
    Ok(joined)
}

fn create_wish_list_url(id: &str) -> Result<Url> {
    let url = Url::parse("https://www.amazon.jp/hz/wishlist/ls/")?;
    let joined = url.join(id)?;
    Ok(joined)
}

pub fn get_wish_list_snapshot(id: &str) -> Result<WishListSnapshot> {
    let url = create_wish_list_url(id)?;
    // let browser = Browser::default()?;
    let mut path = PathBuf::new();
    path.push("bin/chromium");
    let browser = Browser::new(LaunchOptions {
        headless: true,
        sandbox: true,
        idle_browser_timeout: Duration::from_secs(30),
        window_size: None,
        path: Some(path),
        user_data_dir: None,
        port: None,
        ignore_certificate_errors: true,
        extensions: Vec::new(),
        process_envs: None,
        fetcher_options: Default::default(),
        args: Vec::new(),
        disable_default_args: false,
    })?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(url.as_str())?;

    let nav_to_top = tab.wait_for_element("#navBackToTop")?;

    while tab.find_element("#endOfListMarker").is_err() {
        nav_to_top.scroll_into_view()?;
        break;
    }

    let links = tab.find_elements(".a-link-normal")?;
    println!("{:?}", links);
    let mut items: Vec<_> = links
        .iter()
        .filter_map(|link| link.get_attributes().ok())
        .filter_map(|x| x)
        .flatten()
        .filter(|text| text.contains("/dp/"))
        .filter_map(|href| create_item_url(href).ok())
        .collect();
    items.sort();
    items.dedup();

    let snapshot = WishListSnapshot {
        id: id.to_string(),
        url,
        title: String::from(""),
        items,
    };

    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_item_urls() {
        let mut items = vec![
            Url::parse("https://www.amazon.co.jp/dp/B08S7CJV4X/").unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/B08L53NT5P/").unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/B08L54335M/").unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/B08L51YSLR/").unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/B08L5278XF/").unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/B09TPLQGKS/").unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/B0B5Q2RMX6/").unwrap(),
        ];
        items.sort();
        let expected = WishListSnapshot {
            id: String::from("2BDAPI9RQ09E9"),
            url: Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap(),
            title: String::from(""),
            items,
        };
        assert_eq!(
            get_wish_list_snapshot(expected.id.as_str()).unwrap(),
            expected
        );
    }

    #[test]
    fn test_create_item_url() {
        assert_eq!(
            create_item_url("/dp/2BDAPI9RQ09E9/?coliid=IH".to_string()).unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/2BDAPI9RQ09E9/").unwrap()
        );
    }

    #[test]
    fn test_create_wish_list_url() {
        assert_eq!(
            create_wish_list_url("2BDAPI9RQ09E9").unwrap(),
            Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap()
        );
    }
}
