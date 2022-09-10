mod prisma;

use prisma::wish_list;
use prisma::PrismaClient;

#[derive(Debug, PartialEq)]
pub struct ItemMeta {
    id: String,
    url: Url,
}

#[derive(Debug, PartialEq)]
pub struct WishListSnapshot {
    id: String,
    title: String,
    url: Url,
    scraped_at: i32,
    items: Vec<ItemMeta>,
}

#[tokio::main]
async fn main() {
    let client = prisma::new_client().await.unwrap();
}

async fn teeeee(client: PrismaClient, snapshot: WishListSnapshot) {
    let created = client
        .wish_list()
        .upsert(
            wish_list::id::equals(snapshot.id),
            wish_list::create(
                snapshot.id.to_string(),
                snapshot.url.to_string(),
                snapshot.scraped_at,
                snapshot.title,
                vec![],
            ),
            vec![
                wish_list::SetParam::SetScrapedAt(snapshot.scraped_at),
                wish_list::SetParam::SetTitle(snapshot.title.to_string()),
            ],
        )
        .exec()
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
