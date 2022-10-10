use crate::infrastructures::db::prisma::ebook::Data as EbookData;
use crate::infrastructures::db::prisma::PrismaClient;
use anyhow::Result;

pub async fn select_all(client: &PrismaClient) -> Result<Vec<EbookData>> {
    let ebooks = client.ebook().find_many(vec![]).exec().await?;
    Ok(ebooks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructures::db::get_client;

    #[tokio::test]
    async fn test_select_all() {
        dotenv::dotenv().ok();

        let client = get_client().await.unwrap();
        let actual = select_all(&client).await.unwrap();

        assert!(actual.len() > 0)
    }
}
