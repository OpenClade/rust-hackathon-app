mod arena;

#[tokio::main]
async fn main() {
    match arena::get_arena_info().await {
        Ok(body) => {
            println!("Arena Info:\n{}", body);
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
        }
    }
}
