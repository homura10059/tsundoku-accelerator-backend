use anyhow::Result;
use db_client::prisma;
use db_client::prisma::ebook_snapshot::OrderByParam;
use db_client::prisma::wish_list::Data as WishListData;
use db_client::prisma::{ebook, ebook_in_wish_list, wish_list};
use prisma_client_rust::Direction::Desc;

pub async fn select_all_with_snapshot() -> Result<Vec<WishListData>> {
    let client = prisma::new_client().await?;
    let wish_lists = client
        .wish_list()
        .find_many(vec![])
        .with(
            wish_list::ebook_in_wish_list::fetch(vec![]).with(
                ebook_in_wish_list::ebook::fetch()
                    .with(ebook::snapshots::fetch(vec![]).order_by(OrderByParam::ScrapedAt(Desc))),
            ),
        )
        .exec()
        .await?;
    Ok(wish_lists)
}
