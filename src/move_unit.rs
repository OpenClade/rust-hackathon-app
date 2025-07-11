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
    let client = Client::new();
    let url = "https://games-test.datsteam.dev/api/move";
    let token = env::var("API_TOKEN").expect("API_TOKEN not set");

    // Выбираем цель
    let target = decide_target(arena, ant);

    let body = MoveRequest {
        moves: vec![MoveCommand {
            ant: ant.id.clone(),
            path: vec![target],
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

fn decide_target(arena: &ArenaState, ant: &Ant) -> HexCoord {
    // Если несём еду, идем домой
    if ant.food.amount > 0 {
        let home = &arena.spot;
        return step_towards(ant.q, ant.r, home.q, home.r);
    }

    // Ищем ближайшую еду
    if let Some(food) = arena.food.iter().min_by_key(|f| hex_distance(ant.q, ant.r, f.q, f.r)) {
        return step_towards(ant.q, ant.r, food.q, food.r);
    }

    // Нет еды - идем вперед
    HexCoord { q: ant.q + 1, r: ant.r }
}

fn step_towards(q: i32, r: i32, target_q: i32, target_r: i32) -> HexCoord {
    let dq = target_q - q;
    let dr = target_r - r;

    if dq != 0 {
        HexCoord { q: q + dq.signum(), r }
    } else if dr != 0 {
        HexCoord { q, r: r + dr.signum() }
    } else {
        HexCoord { q, r }
    }
}

fn hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs()
        + (q1 + r1 - q2 - r2).abs()
        + (r1 - r2).abs()) / 2
}
