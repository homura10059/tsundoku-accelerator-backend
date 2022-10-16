use url::Url;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Payment {
    pub price: String,
    pub points: String,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct EbookSnapshot {
    pub ebook_id: String,
    pub scraped_at: i64,
    pub thumbnail_url: Url,
    pub payment_ebook: Option<Payment>,
    pub payment_real: Option<Payment>,
}
