mod arena;
mod move_unit;

use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    loop {
        println!("===> Получаем текущее состояние арены...");
        let arena_state = match arena::get_arena_info().await {
            Ok(state) => state,
            Err(e) => {
                eprintln!("Ошибка получения арены: {}", e);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
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

                // Запускаем и сразу дожидаемся (чтобы контролировать RPS)
                match move_unit::move_ant(&arena, &ant).await {
                    Ok(_) => println!("✅ Муравей {} походил.", ant.id),
                    Err(e) => eprintln!("❌ Ошибка при движении {}: {}", ant.id, e),
                }

                // Пауза между запросами (150-200 мс)
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }

        let wait_time = (arena_state.next_turn_in * 1.1).max(0.5);
        println!("Ждем {:.2} сек до следующего хода...\n", wait_time);
        tokio::time::sleep(Duration::from_secs_f32(wait_time)).await;
    }
}
