use axum::Router;
use once_cell::sync::{self, OnceCell};
use std::{net::SocketAddr, time::SystemTime};
use tokio;
mod pages;

#[derive(Debug)]
struct Application {
    start_time: std::time::SystemTime,
}

static APP: OnceCell<Application> = OnceCell::new();

#[tokio::main]
async fn main() {
    APP.set(Application {
        start_time: SystemTime::now(),
    })
    .unwrap();

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
