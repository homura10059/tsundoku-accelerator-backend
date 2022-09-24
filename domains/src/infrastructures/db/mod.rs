pub mod prisma;
pub mod wish_list;

use anyhow::Result;
use prisma::PrismaClient;

pub async fn get_client() -> PrismaClient {
    let client = match prisma::new_client().await {
        Ok(c) => c,
        Err(error) => {
            panic!("Failed to open prisma client : {:?}", error)
        }
    };
    client
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
