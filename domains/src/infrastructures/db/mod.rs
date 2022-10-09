pub mod ebook_snapshot;
pub mod prisma;
pub mod wish_list;

use anyhow::Result;
use prisma::PrismaClient;

pub async fn get_client() -> Result<PrismaClient> {
    let client = prisma::new_client().await?;
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[tokio::test]
    async fn test_get_client() {
        dotenv::dotenv().ok();

        let client = get_client().await;
    }
}
