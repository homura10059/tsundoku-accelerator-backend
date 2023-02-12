use crate::item_metadata::ItemMetaData;
use url::Url;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct WishListSnapshot {
    pub id: String,
    pub title: String,
    pub url: Url,
    pub scraped_at: i64,
    pub items: Vec<ItemMetaData>,
}
