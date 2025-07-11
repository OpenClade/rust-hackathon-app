use reqwest::Client;
use std::env;

pub async fn get_arena_info() -> Result<String, reqwest::Error> {
    // Загружаем переменные окружения из .env
    dotenv::dotenv().ok();

    let client = Client::new();

    let url = "https://games-test.datsteam.dev/api/arena";
    let token = env::var("API_TOKEN").expect("API_TOKEN not set");

    let resp = client
        .get(url)
        .header("accept", "application/json")
        .header("X-Auth-Token", token)
        .send()
        .await?;

    let body = resp.text().await?;
    Ok(body)
}
