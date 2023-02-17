mod repositories;

use crate::ebook_snapshots::snap_ebook;
use anyhow::Result;
use futures::stream;
use futures::StreamExt;
use headless_chrome::Browser;
use infrastructures::prisma;

pub async fn snap_all_ebook() -> Result<()> {
    let client = prisma::new_client().await?;
    let lists = repositories::select_all(&client).await?;
    let browser = Browser::default()?;

    let futures = lists
        .into_iter()
        .map(|ebook| snap_ebook(&client, &browser, ebook.id))
        .collect::<Vec<_>>();
    let stream = stream::iter(futures).buffer_unordered(3);
    stream.collect::<Vec<_>>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    // #[tokio::test] // 必要な時だけ動かす
    // async fn it_works_snap_all_ebook() {
    //     dotenv::dotenv().ok();
    //
    //     let actual = snap_all_ebook().await.unwrap();
    //     assert_eq!(actual, ());
    // }
}
