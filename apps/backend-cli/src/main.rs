use clap::{Parser, Subcommand};
use domains::notifications;
use domains::wish_lists;
use dotenv;
use std::result;

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
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();

    match args.command {
        Commands::UpdateAllWishlist => {
            let result = wish_lists::update_all_wish_list().await;
            match result {
                Ok(_) => {
                    // 成功してれば通知しない
                }
                Err(e) => {
                    notifications::send_alert_message(e.to_string())
                        .await
                        .unwrap();
                    ()
                }
            }
        }
        Commands::SendNotification => {
            let data = wish_lists::services::select_all_wish_list_and_snapshot()
                .await
                .unwrap();
            for d in data {
                notifications::notify(&d).await.unwrap();
            }
        }
    }
}
