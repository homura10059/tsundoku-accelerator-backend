use crate::item_metadata::ItemMetaData;
use crate::models::WishListSnapshot;
use anyhow::Result;
use headless_chrome::{Browser, Element};
use pure_funcs::get_now_in_sec;
use url::Url;

pub fn search_from(attributes: &Vec<String>, key: &str) -> Option<String> {
    let target = attributes.iter().position(|attr| attr == key)? + 1;
    let (_pos, attr) = attributes
        .iter()
        .enumerate()
        .find(|(pos, _attr)| pos.eq(&target))?;
    let result = attr.clone();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_from_returns_some() {
        let mock_vec = vec!["key_1".to_string(), "val_1".to_string()];
        let actual = search_from(&mock_vec, "key_1");
        assert_eq!(actual, Some("val_1".to_string()))
    }

    #[test]
    fn search_from_returns_none() {
        let mock_vec = vec!["key_1".to_string(), "val_1".to_string()];
        let actual = search_from(&mock_vec, "key_999");
        assert_eq!(actual, None)
    }

    #[test]
    fn search_from_returns_none2() {
        let mock_vec = vec!["key_1".to_string(), "val_1".to_string()];
        let actual = search_from(&mock_vec, "val_1");
        assert_eq!(actual, None)
    }
}
