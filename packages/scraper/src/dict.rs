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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_2() {
        let mock_vec = vec!["key_1".to_string(), "val_1".to_string()];
        let actual = from(&mock_vec);

        let mut expected = HashMap::new();
        expected.insert("key_1".to_string(), "val_1".to_string());
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_from_3() {
        let mock_vec = vec![
            "key_1".to_string(),
            "val_1".to_string(),
            "key_2".to_string(),
        ];
        let actual = from(&mock_vec);

        let mut expected = HashMap::new();
        expected.insert("key_1".to_string(), "val_1".to_string());
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_from_4() {
        let mock_vec = vec![
            "key_1".to_string(),
            "val_1".to_string(),
            "key_2".to_string(),
            "val_2".to_string(),
        ];
        let actual = from(&mock_vec);

        let mut expected = HashMap::new();
        expected.insert("key_1".to_string(), "val_1".to_string());
        expected.insert("key_2".to_string(), "val_2".to_string());
        assert_eq!(actual, expected)
    }
}
