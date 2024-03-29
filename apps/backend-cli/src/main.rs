#[macro_use]
extern crate log;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

use comannds::domains::ebooks;
use comannds::domains::notifications;
use comannds::domains::wish_lists::services;
use queries::wish_list::select_all_with_snapshot;

use crate::Commands::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
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
    dotenv().ok();
    env_logger::init();

    let args = Args::parse();

    info!("{:?} : start", args.command);
    match args.command {
        UpdateAllWishlist => {
            services::update_all_wish_list()
                .await
                .expect("can not update");
        }
        SendNotification => {
            let data = select_all_with_snapshot().await.unwrap();
            for d in data {
                notifications::notify(&d).await.unwrap();
            }
        }
        SnapEbooks => {
            ebooks::snap_all_ebook().await.expect("can not snap");
        }
        AllFlow => {
            services::update_all_wish_list()
                .await
                .expect("can not update");
            ebooks::snap_all_ebook().await.expect("can not snap");
            let data = select_all_with_snapshot().await.unwrap();
            for d in data {
                notifications::notify(&d).await.unwrap();
            }
        }
    }
    info!("{:?} : finish", args.command);
}
