use anyhow::Context;

pub async fn get(url: impl reqwest::IntoUrl) -> anyhow::Result<String> {
    let session = std::env::var("SESSION").context("SESSION is not defined")?;
    let cookie = format!("session={}", session);

    let response = reqwest::Client::default()
        .get(url)
        .header("cookie", cookie)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(response)
}
