use crate::infrastructures::prisma::ebook::Data as EBookData;
use crate::infrastructures::prisma::ebook_in_wish_list::Data as EBookInWishListData;
use crate::infrastructures::prisma::wish_list::Data as WishListData;
use anyhow::{anyhow, Result};
use envy;
use serde::Deserialize;
use webhook::client::{WebhookClient, WebhookResult};
use webhook::models::Message;

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
            message.content(text.as_ref())
        })
        .await
        .unwrap();

    Ok(result)
}

pub async fn notify(data: &WishListData) -> Result<()> {
    let ebook_in_wish_list = data
        .ebook_in_wish_list
        .clone()
        .ok_or(anyhow!("no ebooks"))?;
    let ebooks = ebook_in_wish_list
        .iter()
        .filter_map(|x| x.ebook.clone())
        .collect::<Vec<_>>();

    let mut message = Message::new();
    message.username("bot");
    message.avatar_url(
        "https://github.com/homura10059/sophia-bot/blob/master/image/P5S_icon_sophia.png?raw=true",
    );
    message.content("test");

    let url: &str = "https://discord.com/api/webhooks/865929593491161138/IaLTBUVImvSmlzS5zdms860Cvha_wefwi21VMZEjpzzta8-smN4AUadUgrMDapeIQsIE";
    let client: WebhookClient = WebhookClient::new(url);
    client.send_message(&message).await.unwrap();
    //
    // client
    //     .send(|message| {
    //         // for ebook in ebooks {
    //         //     let snap = ebook.snapshots;
    //         // }
    //         message.embed(|embed| {
    //             embed
    //                 .title(data.title.as_str())
    //                 .url(data.url.as_str())
    //                 .field("price", "data", false)
    //         })
    //     })
    //     .await
    //     .unwrap();
    Ok(())
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
        let ebook = EBookData {
            id: "id".to_string(),
            url: "url".to_string(),
            title: "title".to_string(),
            price: 42.0,
            snapshots: None,
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
        assert_eq!(actual, ())
    }
}
