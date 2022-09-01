mod prisma;

use prisma::user;

#[tokio::main]
async fn main() {
    let client= prisma::new_client().await.unwrap();

    let users = client.user()
        .find_many(vec![user::display_name::equals("homura".to_string())])
        .exec().await.unwrap();

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
