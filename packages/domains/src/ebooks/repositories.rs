use anyhow::Result;
use infrastructures::prisma::ebook::Data as EbookData;
use infrastructures::prisma::PrismaClient;

pub async fn select_all(client: &PrismaClient) -> Result<Vec<EbookData>> {
    let ebooks = client.ebook().find_many(vec![]).exec().await?;
    Ok(ebooks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use infrastructures::prisma;

    #[tokio::test]
    async fn test_select_all() {
        dotenv::dotenv().ok();

        let client = prisma::new_client().await.unwrap();
        let actual = select_all(&client).await.unwrap();

        assert!(actual.len() > 0)
    }
}

pub mod db {}
