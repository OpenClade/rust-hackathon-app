mod arena;
mod move_unit;

use futures::future::join_all;
use std::time::Duration;
use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    loop {
        println!("===> Получаем текущее состояние арены...");
        let arena_state = match arena::get_arena_info().await {
            Ok(state) => state,
            Err(e) => {
                // Проверяем, это ошибка декодирования
                let is_decode = e.is_decode();

                eprintln!("Ошибка получения арены: {}", e);

                if is_decode {
                    println!("Похоже, вы не зарегистрированы. Запускаем регистрацию...");
                    try_register_forever().await;
                    // После регистрации пробуем заново
                    continue;
                } else {
                    // Иные ошибки (например, сеть) - подождать и пробовать снова
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
            }
        };

        println!(
            "Ход №{} | найдено {} муравьев",
            arena_state.turn_no,
            arena_state.ants.len()
        );

        if arena_state.ants.is_empty() {
            println!("Нет муравьев. Ждем появления...");
        } else {
            for ant in &arena_state.ants {
                let arena = arena_state.clone();
                let ant = ant.clone();

                match move_unit::move_ant(&arena, &ant).await {
                    Ok(_) => println!("✅ Муравей {} походил.", ant.id),
                    Err(e) => eprintln!("❌ Ошибка при движении {}: {}", ant.id, e),
                }

                // Ограничение частоты запросов
                tokio::time::sleep(Duration::from_millis(160)).await;
            }
        }

        let wait_time = (arena_state.next_turn_in * 1.1).max(0.5);
        println!("Ждем {:.2} сек до следующего хода...\n", wait_time);
        tokio::time::sleep(Duration::from_secs_f32(wait_time)).await;
    }
}

async fn try_register_forever() {
    let client = reqwest::Client::new();
    let url = "https://games-test.datsteam.dev/api/register";
    let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");

    loop {
        println!("Пробуем зарегистрироваться...");

        let resp = client
            .post(url)
            .header("accept", "application/json")
            .header("X-Auth-Token", &token)
            .send()
            .await;

        match resp {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ Успешно зарегистрированы.");
                    return;
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    eprintln!("❌ Сервер вернул ошибку регистрации {}: {}", status, text);
                }
            }
            Err(e) => {
                eprintln!("❌ Ошибка запроса регистрации: {}", e);
            }
        }

        // Ждём перед новой попыткой
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
