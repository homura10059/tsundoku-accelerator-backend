use anyhow::Result;
use db_client::prisma::ebook::Data as EbookData;
use db_client::prisma::PrismaClient;

pub async fn select_all(client: &PrismaClient) -> Result<Vec<EbookData>> {
    let ebooks = client.ebook().find_many(vec![]).exec().await?;
    Ok(ebooks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use db_client::prisma;

    #[tokio::test]
    async fn test_select_all() {
        dotenv::dotenv().ok();

        let client = prisma::new_client().await.unwrap();
        let actual = select_all(&client).await.unwrap();

        assert!(actual.len() > 0)
    }
}

pub mod db {}
