use std::io::BufRead;

mod challenge;
mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().count() > 1 {
        run(std::env::args().skip(1).map(Ok)).await
    } else {
        run(std::io::stdin().lock().lines()).await
    }
}

async fn run(args: impl Iterator<Item = std::io::Result<String>>) -> anyhow::Result<()> {
    for arg in args {
        challenge::solve(&arg?.parse()?).await?;
    }

    Ok(())
}
