use crate::infrastructures::db::prisma::PrismaClient;
use crate::infrastructures::db::wish_list;
use crate::infrastructures::{db, scraper};
use anyhow::Result;
use futures::future;

pub async fn update_wish_list(client: &PrismaClient, id: &str) -> Result<()> {
    let snapshot = scraper::get_wish_list_snapshot(id)?;
    let result = wish_list::upsert_wish_list(client, &snapshot).await?;
    Ok(result)
}

pub async fn update_all_wish_list() -> Result<()> {
    let id_list = vec![
        "2BDAPI9RQ09E9",
        "2VJWMFFVPSTEP",
        "1CU482XA9HO5F",
        "3MLOGSF4M4JP",
        "3E1B11VRAD7UZ",
        "1KERB2C8N5NGY",
        "211LVEVFIA3JT",
    ];
    let client = db::get_client().await;

    let results = id_list
        .into_iter()
        .map(|id| update_wish_list(&client, id))
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

        let actual = update_wish_list(&client, "2BDAPI9RQ09E9").await.unwrap();
        assert_eq!(actual, ());
    }

    #[tokio::test]
    async fn it_works_update_all_wish_list() {
        dotenv::dotenv().ok();

        let actual = update_all_wish_list().await.unwrap();
        assert_eq!(actual, ());
    }
}
