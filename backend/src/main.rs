use axum::Router;
use std::net::SocketAddr;
use tokio;
mod pages;
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        let app = pages::configure_route(Router::new());
        let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let mut buffer = String::new();
    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();
        if !handle_command(buffer.lines().next().unwrap()) {
            handle.abort();
            break;
        }
    }
}

fn handle_command(args: &str) -> bool {
    match args {
        "q" => false,
        _ => {
            println!("no such command");
            true
        }
    }
}
