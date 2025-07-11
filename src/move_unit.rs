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
    let directions = [
        (1, 0), (1, -1), (0, -1),
        (-1, 0), (-1, 1), (0, 1),
    ];

    let mut path = Vec::new();
    let mut current_q = q;
    let mut current_r = r;

    for _ in 0..steps {
        if current_q == target_q && current_r == target_r {
            break;
        }

        let mut best = None;
        let mut best_dist = i32::MAX;

        for (dq, dr) in directions {
            let nq = current_q + dq;
            let nr = current_r + dr;
            let dist = hex_distance(nq, nr, target_q, target_r);

            if dist < best_dist {
                best_dist = dist;
                best = Some((nq, nr));
            }
        }

        if let Some((nq, nr)) = best {
            current_q = nq;
            current_r = nr;
            path.push(HexCoord { q: nq, r: nr });
        } else {
            break;
        }
    }

    path
}


fn hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs()
        + (q1 + r1 - q2 - r2).abs()
        + (r1 - r2).abs()) / 2
}
