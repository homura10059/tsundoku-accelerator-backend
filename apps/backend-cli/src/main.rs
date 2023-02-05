use clap::{Parser, Subcommand};
use dotenv;

use domains::ebooks;
use domains::notifications;
use domains::wish_lists;

use crate::Commands::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// update all wishlists.
    UpdateAllWishlist,
    /// send notification for all ebooks
    SendNotification,
    /// snap ebook data
    SnapEbooks,
    /// exec all work flow
    AllFlow,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();

    match args.command {
        UpdateAllWishlist => {
            wish_lists::update_all_wish_list()
                .await
                .expect("can not update");
        }
        SendNotification => {
            let data = wish_lists::services::select_all_wish_list_and_snapshot()
                .await
                .unwrap();
            for d in data {
                notifications::notify(&d).await.unwrap();
            }
        }
        SnapEbooks => {
            ebooks::snap_all_ebook().await.expect("can not snap");
        }
        AllFlow => {
            wish_lists::update_all_wish_list()
                .await
                .expect("can not update");
            ebooks::snap_all_ebook().await.expect("can not snap");
            let data = wish_lists::services::select_all_wish_list_and_snapshot()
                .await
                .unwrap();
            for d in data {
                notifications::notify(&d).await.unwrap();
            }
        }
    }
}
