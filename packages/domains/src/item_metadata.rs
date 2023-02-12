use url::Url;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct ItemMetaData {
    pub id: String,
    pub url: Url,
    pub title: String,
    pub price: String,
}

impl ItemMetaData {
    fn create_url<T: AsRef<str>>(href: T) -> anyhow::Result<Url> {
        let url = Url::parse("https://www.amazon.co.jp")?;
        let mut joined = url.join(href.as_ref())?;
        joined.set_query(None);
        Ok(joined)
    }

    pub fn new<T: Into<String>>(href: T, title: T, price: T) -> anyhow::Result<ItemMetaData> {
        let url = ItemMetaData::create_url(href.into())?;
        let path = url.path().to_string();
        let tmp: Vec<_> = path.split('/').collect();
        let id = tmp.get(2).unwrap();
        let meta = ItemMetaData {
            id: id.to_string(),
            url,
            title: title.into(),
            price: price.into(),
        };
        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let expected = ItemMetaData {
            id: String::from("2BDAPI9RQ09E9"),
            url: Url::parse("https://www.amazon.co.jp/dp/2BDAPI9RQ09E9/").unwrap(),
            title: String::from("title"),
            price: String::from("100"),
        };
        assert_eq!(
            ItemMetaData::new(
                "/dp/2BDAPI9RQ09E9/?coliid=IH".to_string(),
                String::from("title"),
                String::from("100")
            )
            .unwrap(),
            expected
        );
    }
}
