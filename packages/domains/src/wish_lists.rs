mod repositories;
pub mod services;

use anyhow::Result;
use futures::stream;
use futures::StreamExt;
use infrastructures::prisma;
use infrastructures::prisma::PrismaClient;

pub async fn update_wish_list(client: &PrismaClient, id: String) -> Result<()> {
    let snapshot = repositories::get_wish_list_snapshot(id.as_str())?;
    repositories::upsert_wish_list(client, &snapshot).await?;
    Ok(())
}

pub async fn update_all_wish_list() -> Result<()> {
    let client = prisma::new_client().await?;
    let lists = repositories::select_all_wish_list(&client).await?;

    let futures = lists
        .into_iter()
        .map(|list| update_wish_list(&client, list.id))
        .collect::<Vec<_>>();
    let stream = stream::iter(futures).buffer_unordered(3);
    stream.collect::<Vec<_>>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[tokio::test]
    async fn it_works_update_wish_list() {
        dotenv::dotenv().ok();
        let client = prisma::new_client().await.unwrap();

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
