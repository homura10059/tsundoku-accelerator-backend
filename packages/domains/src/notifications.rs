use crate::infrastructures::prisma::ebook::Data as EBookData;
use crate::infrastructures::prisma::ebook_in_wish_list::Data as EBookInWishListData;
use crate::infrastructures::prisma::ebook_snapshot::Data as EBookSnapShotData;
use crate::infrastructures::prisma::wish_list::Data as WishListData;
use anyhow::{anyhow, Result};
use chrono::{DateTime, FixedOffset, LocalResult, TimeZone};
use envy;
use serde::Deserialize;
use webhook::client::{WebhookClient, WebhookResult};
use webhook::models::{Embed, Message};

#[derive(Deserialize, Debug)]
struct Config {
    bot_name: String,
    avatar_url: String,
    alert_chanel: String,
    sale_chanel: String,
}

pub async fn send_alert_message<T: AsRef<str>>(text: T) -> Result<bool> {
    let config = envy::prefixed("DISCORD_").from_env::<Config>()?;

    let client: WebhookClient = WebhookClient::new(config.alert_chanel.as_ref());
    let result = client
        .send(|message| {
            message.username(config.bot_name.as_ref());
            message.avatar_url(config.avatar_url.as_ref());
            message.content(text.as_ref());
            message
        })
        .await
        .unwrap();

    Ok(result)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum EmbedColor {
    Red = 15548997,
    Green = 5763719,
    Yellow = 16776960,
    Grey = 9807270,
}

impl Into<String> for EmbedColor {
    fn into(self) -> String {
        (self as i32).to_string()
    }
}

fn get_color(snapshot: &EBookSnapShotData) -> EmbedColor {
    let discount_rate = snapshot.discount_rate.unwrap_or(0.0);
    let points_rate = snapshot.points_rate;

    if !(discount_rate >= 20.0 || points_rate >= 20.0) {
        return EmbedColor::Grey;
    }

    if discount_rate >= 35.0 || points_rate >= 35.0 {
        return EmbedColor::Red;
    }

    if discount_rate >= 30.0 || points_rate >= 30.0 {
        return EmbedColor::Yellow;
    }

    return EmbedColor::Green;
}

#[derive(Debug)]
struct EmbedItem {
    title: String,
    url: String,
    color: EmbedColor,
    price: String,
    discount_rate: String,
    points_rate: String,
    update_datetime: String,
}

#[derive(Debug)]
struct SaleNotification {
    to: String,
    from_user_name: String,
    from_avatar_url: String,
    content: String,
    embeds: Vec<EmbedItem>,
}

impl SaleNotification {
    fn new(data: &WishListData) -> Option<SaleNotification> {
        let config = envy::prefixed("DISCORD_").from_env::<Config>().ok()?;
        let ebook_in_wish_list = data.ebook_in_wish_list.clone()?;
        let embeds = ebook_in_wish_list
            .iter()
            .filter_map(|x| x.ebook.clone())
            .filter_map(|ebook| EmbedItem::new(&ebook))
            .collect::<Vec<_>>();
        Some(SaleNotification {
            to: config.sale_chanel,
            from_user_name: config.bot_name,
            from_avatar_url: config.avatar_url,
            content: format!("{} のセール情報", data.title),
            embeds,
        })
    }
}

impl EmbedItem {
    fn new(ebook: &EBookData) -> Option<EmbedItem> {
        let snapshots = ebook.snapshots.clone()?;
        let latest_snapshot = snapshots.first()?;
        let offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let date = offset.timestamp_opt(latest_snapshot.scraped_at, 0).unwrap();

        Some(EmbedItem {
            title: ebook.title.clone(),
            url: ebook.url.clone(),
            color: get_color(latest_snapshot),
            price: format!("¥{}", latest_snapshot.price),
            discount_rate: format!("{}%", latest_snapshot.discount_rate.unwrap_or(0.0)),
            points_rate: format!("{}%", latest_snapshot.points_rate),
            update_datetime: date.format("%Y/%m/%d %H:%M:%S %Z").to_string(),
        })
    }
}

fn convert_from(data: &WishListData) -> Option<Vec<Message>> {
    let notification = SaleNotification::new(data)?;

    let chunks = notification.embeds.chunks(10).collect::<Vec<_>>();

    let result = chunks
        .iter()
        .map(|items| {
            let mut message = Message::new();
            message.username(notification.from_user_name.as_str());
            message.avatar_url(notification.from_avatar_url.as_str());
            message.content(notification.content.as_str());

            for item in *items {
                let color: String = item.color.into();
                message.embed(move |embed| {
                    embed
                        .title(item.title.as_str())
                        .url(item.url.as_ref())
                        .color(color.as_str())
                        .field("金額", item.price.as_str(), true)
                        .field("値引き率", item.discount_rate.as_str(), true)
                        .field("ポイント還元率", item.points_rate.as_str(), true)
                        .field("更新日", item.update_datetime.as_str(), true)
                });
            }
            message
        })
        .collect::<Vec<_>>();

    Some(result)
}

pub async fn notify(data: &WishListData) -> Result<bool> {
    let config = envy::prefixed("DISCORD_").from_env::<Config>()?;

    let messages = convert_from(data).ok_or(anyhow!("can not create message"))?;
    let client: WebhookClient = WebhookClient::new(config.sale_chanel.as_ref());
    for message in messages {
        client.send_message(&message).await.unwrap();
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    extern crate dotenv;

    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn can_send_alert() {
        dotenv().ok();

        let actual = send_alert_message("unit-test").await.unwrap();
        assert_eq!(actual, true)
    }

    #[tokio::test]
    async fn test_notify() {
        dotenv().ok();

        let snapshot = EBookSnapShotData {
            id: "".to_string(),
            ebook: None,
            ebook_id: "".to_string(),
            scraped_at: 0,
            thumbnail_url: "".to_string(),
            price: 42.0,
            discount: None,
            discount_rate: None,
            points: 1.0,
            points_rate: 0.10,
        };

        let ebook = EBookData {
            id: "id".to_string(),
            url: "https://example.com".to_string(),
            title: "title".to_string(),
            price: 42.0,
            snapshots: Some(vec![snapshot]),
            ebook_in_wish_list: None,
        };

        let ebook_in_wish_list = EBookInWishListData {
            wish_list: None,
            wish_list_id: "id".to_string(),
            ebook: Some(Box::new(ebook)),
            ebook_id: "ebook_id".to_string(),
        };

        let data = WishListData {
            id: "id".to_string(),
            url: "url".to_string(),
            scraped_at: 0,
            title: "title".to_string(),
            ebook_in_wish_list: Some(vec![ebook_in_wish_list]),
        };
        let actual = notify(&data).await.unwrap();
        assert_eq!(actual, true)
    }
}
