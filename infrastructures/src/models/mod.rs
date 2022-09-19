use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct ItemMetaData {
    pub id: String,
    pub url: Url,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WishListSnapshot {
    pub id: String,
    pub title: String,
    pub url: Url,
    pub scraped_at: i64,
    pub items: Vec<ItemMetaData>,
}
