use crate::models::ItemMetaData;
use anyhow::Result;
use url::Url;

fn create_item_url(href: String) -> Result<Url> {
    let url = Url::parse("https://www.amazon.co.jp")?;
    let mut joined = url.join(href.as_str())?;
    joined.set_query(None);
    Ok(joined)
}

pub fn create(href: String, title: String, price: String) -> Result<ItemMetaData> {
    let url = create_item_url(href)?;
    let path = url.path().to_string();
    let a: Vec<_> = path.split("/").collect();
    let id = a.get(2).unwrap();
    let meta = ItemMetaData {
        id: id.to_string(),
        url,
        title,
        price,
    };
    Ok(meta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_item_url() {
        assert_eq!(
            create_item_url("/dp/2BDAPI9RQ09E9/?coliid=IH".to_string()).unwrap(),
            Url::parse("https://www.amazon.co.jp/dp/2BDAPI9RQ09E9/").unwrap()
        );
    }

    #[test]
    fn test_create() {
        let expected = ItemMetaData {
            id: String::from("2BDAPI9RQ09E9"),
            url: Url::parse("https://www.amazon.co.jp/dp/2BDAPI9RQ09E9/").unwrap(),
            title: String::from("title"),
            price: String::from("100"),
        };
        assert_eq!(
            create(
                "/dp/2BDAPI9RQ09E9/?coliid=IH".to_string(),
                String::from("title"),
                String::from("100")
            )
            .unwrap(),
            expected
        );
    }
}
