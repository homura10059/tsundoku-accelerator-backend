use crate::models::WishListSnapshot;
use anyhow::Result;
use url::Url;

pub fn create_url(id: &str) -> Result<Url> {
    let url = Url::parse("https://www.amazon.jp/hz/wishlist/ls/")?;
    let joined = url.join(id)?;
    Ok(joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_url() {
        assert_eq!(
            create_url("2BDAPI9RQ09E9").unwrap(),
            Url::parse("https://www.amazon.jp/hz/wishlist/ls/2BDAPI9RQ09E9").unwrap()
        );
    }
}
