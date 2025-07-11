use serde::Serialize;
use std::env;

use crate::arena::{ArenaState, Ant};
use reqwest::Client;

#[derive(Serialize)]
struct MoveRequest {
    moves: Vec<MoveCommand>,
}

#[derive(Serialize)]
struct HexCoord {
    q: i32,
    r: i32,
}

#[derive(Serialize)]
struct MoveCommand {
    ant: String,
    path: Vec<HexCoord>,
}

pub async fn move_ant(
    arena: &ArenaState,
    ant: &Ant,
) -> Result<(), Box<dyn std::error::Error>> {
    // Загружаем .env
    dotenv::dotenv().ok();

    let client = Client::new();
    let url = "https://games-test.datsteam.dev/api/move";
    let token = env::var("API_TOKEN").expect("API_TOKEN not set");

    // Строим путь из нескольких шагов
    let path = decide_target(arena, ant);

    let body = MoveRequest {
        moves: vec![MoveCommand {
            ant: ant.id.clone(),
            path,
        }],
    };

    let resp = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .header("X-Auth-Token", token)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!("HTTP error {}: {}", status, text).into())
    }
}

fn decide_target(arena: &ArenaState, ant: &Ant) -> Vec<HexCoord> {
    let steps = 5; // сколько клеток пройти за один запрос

    if ant.food.amount > 0 {
        let home = &arena.spot;
        return build_path(ant.q, ant.r, home.q, home.r, steps);
    }

    if let Some(food) = arena.food.iter().min_by_key(|f| hex_distance(ant.q, ant.r, f.q, f.r)) {
        return build_path(ant.q, ant.r, food.q, food.r, steps);
    }

    // Нет еды — идем вперед
    let mut path = Vec::new();
    for i in 1..=steps {
        path.push(HexCoord { q: ant.q + i as i32, r: ant.r });
    }
    path
}

fn build_path(q: i32, r: i32, target_q: i32, target_r: i32, steps: usize) -> Vec<HexCoord> {
    let mut path = Vec::new();
    let mut current_q = q;
    let mut current_r = r;

    for _ in 0..steps {
        let dq = target_q - current_q;
        let dr = target_r - current_r;

        if dq == 0 && dr == 0 {
            break;
        }

        let next = if dq.abs() >= dr.abs() {
            HexCoord {
                q: current_q + dq.signum(),
                r: current_r,
            }
        } else {
            HexCoord {
                q: current_q,
                r: current_r + dr.signum(),
            }
        };

        current_q = next.q;
        current_r = next.r;
        path.push(next);
    }

    path
}

fn hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs()
        + (q1 + r1 - q2 - r2).abs()
        + (r1 - r2).abs()) / 2
}
