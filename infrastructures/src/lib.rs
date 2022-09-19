mod db;
mod models;

#[tokio::main]
async fn main() {
    // let client = db::prisma::new_client().await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
