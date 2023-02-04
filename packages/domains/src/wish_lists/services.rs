use crate::infrastructures::prisma;
use crate::infrastructures::prisma::ebook_snapshot::OrderByParam;
use crate::infrastructures::prisma::wish_list::Data as WishListData;
use crate::infrastructures::prisma::{ebook, ebook_in_wish_list, wish_list, PrismaClient};

use anyhow::Result;
use prisma_client_rust::Direction::Desc;

pub async fn select_all_wish_list_and_snapshot() -> Result<Vec<WishListData>> {
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
