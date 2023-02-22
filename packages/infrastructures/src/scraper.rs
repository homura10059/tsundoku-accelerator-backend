use std::collections::HashMap;

pub fn from(attr: &[String]) -> HashMap<String, String> {
    attr.chunks(2)
        .filter_map(|chunk| {
            let key = chunk.get(0)?;
            let val = chunk.get(1)?;
            Some((key.to_string(), val.to_string()))
        })
        .collect::<HashMap<String, String>>()
}

pub fn search_from(attributes: &[String], key: &str) -> Option<String> {
    let dict = from(attributes);
    let val = dict.get(key).map(|x| x.to_string());
    val
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
