use crate::infrastructures::db::prisma::PrismaClient;
use crate::infrastructures::{db, scraper};
use anyhow::Result;
use futures::future;

pub async fn update_wish_list(client: &PrismaClient, id: String) -> Result<()> {
    let snapshot = scraper::get_wish_list_snapshot(id.as_str())?;
    let result = db::wish_list::upsert_wish_list(client, &snapshot).await?;
    Ok(result)
}

pub async fn update_all_wish_list() -> Result<()> {
    let client = db::get_client().await;
    let lists = db::wish_list::select_all_wish_list(&client).await?;

    let results = lists
        .into_iter()
        .map(|list| update_wish_list(&client, list.id))
        .collect::<Vec<_>>();
    future::join_all(results).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[tokio::test]
    async fn it_works_update_wish_list() {
        dotenv::dotenv().ok();
        let client = db::get_client().await;

        let actual = update_wish_list(&client, String::from("2BDAPI9RQ09E9"))
            .await
            .unwrap();
        assert_eq!(actual, ());
    }

    #[tokio::test]
    async fn it_works_update_all_wish_list() {
        dotenv::dotenv().ok();

        let actual = update_all_wish_list().await.unwrap();
        assert_eq!(actual, ());
    }
}
