// use crate::infrastructures::prisma::wish_list::Data as WishListData;
// use anyhow::{Result, anyhow};
// use webhook::client::WebhookClient;
//
// pub async fn notify(data: &WishListData) -> Result<()> {
//     let url: &str = "https://discord.com/api/webhooks/865929593491161138/IaLTBUVImvSmlzS5zdms860Cvha_wefwi21VMZEjpzzta8-smN4AUadUgrMDapeIQsIE";
//     let ebook_in_wish_list = data.ebook_in_wish_list.ok_or(anyhow!("no ebooks"))?;
//     let ebooks = ebook_in_wish_list.iter().filter_map(|x| x.ebook).collect::<Vec<_>>();
//
//     let client: WebhookClient = WebhookClient::new(url);
//     let result = client
//         .send(|message| {
//             for ebook in ebooks {
//                 let snap = ebook.snapshots;
//
//             }
//             message.embed(|embed| {
//                 embed
//                     .title(data.title.as_str())
//                     .url(data.url.as_str())
//                     .field("price", data., false)
//             })
//         })
//         .await
//         .unwrap();
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_notify() {
//         let data = WishListData {
//             id: "id".to_string(),
//             url: "url".to_string(),
//             scraped_at: 0,
//             title: "title".to_string(),
//             ebook_in_wish_list: None
//         };
//         let actual = notify(&data).await.unwrap();
//         assert_eq!(actual, ())
//     }
// }
