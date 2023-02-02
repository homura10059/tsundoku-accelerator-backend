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

enum EmbedColor {
    Red = 15548997,
    Green = 5763719,
    Yellow = 16776960,
    Grey = 9807270,
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

fn convert_from(data: &WishListData) -> Result<Vec<Message>> {
    let config = envy::prefixed("DISCORD_").from_env::<Config>()?;

    let message_title = format!("{} のセール情報", data.title);

    let ebook_in_wish_list = data
        .ebook_in_wish_list
        .clone()
        .ok_or(anyhow!("no ebooks"))?;
    let ebooks = ebook_in_wish_list
        .iter()
        .filter_map(|x| x.ebook.clone())
        .collect::<Vec<_>>();
    let chunks = ebooks.chunks(10).collect::<Vec<_>>();
    let result = chunks
        .iter()
        .map(|ebooks| {
            let mut message = Message::new();
            message.username(config.bot_name.as_ref());
            message.avatar_url(config.avatar_url.as_ref());
            message.content(message_title.as_ref());

            for ebook in *ebooks {
                if ebook.snapshots.is_none() {
                    continue;
                }
                let snapshots = ebook.snapshots.clone().unwrap();
                let latest_snapshot = snapshots.get(0);
                if latest_snapshot.is_none() {
                    continue;
                }
                let snap = latest_snapshot.unwrap();
                message.embed(|embed| {
                    let offset = FixedOffset::east_opt(9 * 3600).unwrap();
                    let date = offset.timestamp_opt(snap.scraped_at, 0).unwrap();
                    let color = get_color(snap) as isize;

                    embed
                        .title(ebook.title.as_str())
                        .url(ebook.url.as_ref())
                        .color(color.to_string().as_ref())
                        .field("金額", format!("¥{}", snap.price).as_ref(), true)
                        .field(
                            "値引き率",
                            format!("{}%", snap.discount_rate.unwrap_or(0.0)).as_ref(),
                            true,
                        )
                        .field(
                            "ポイント還元率",
                            format!("{}%", snap.points_rate).as_ref(),
                            true,
                        )
                        .field(
                            "更新日",
                            date.format("%Y/%m/%d %H:%M:%S %Z").to_string().as_ref(),
                            true,
                        )
                });
            }
            message
        })
        .collect::<Vec<_>>();

    Ok(result)
}

pub async fn notify(data: &WishListData) -> Result<bool> {
    let config = envy::prefixed("DISCORD_").from_env::<Config>()?;

    let messages = convert_from(data)?;
    let client: WebhookClient = WebhookClient::new(config.alert_chanel.as_ref());
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
