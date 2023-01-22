use clap::{Parser, Subcommand};
use domains::wish_lists;
use dotenv;

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
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();

    match args.command {
        Commands::UpdateAllWishlist => wish_lists::update_all_wish_list().await.unwrap(),
    }
}
