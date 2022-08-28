mod prisma;

use prisma::user;

#[tokio::main]
async fn main() {
    let client= prisma::new_client().await?;

    let users = client.user()
        .find_many(vec![user::display_name::equals("homura".to_string())])
        .exec().await?;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
