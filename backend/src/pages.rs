use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Error, Router,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::Lazy;

use crate::APP;
use std::io::prelude::*;
use std::{fs::File, time::Duration};

static ROOT_PAGE: Lazy<String> = Lazy::new(|| {
    let mut buf = String::new();
    File::open("assets/index.html")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    buf
});

pub fn configure_route(router: Router) -> Router {
    router
        .route("/", get(root))
        .route("/display_socket", get(display_socket))
}

async fn root() -> Html<&'static str> {
    Html(&*ROOT_PAGE)
}

async fn display_socket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(display_connection)
}

async fn display_connection(socket: WebSocket) {
    let (sender, receiver) = socket.split();

    let result = tokio::select! {
        v = receiving(receiver) => v,
        v = sending(sender) => v,
    };
}

async fn receiving(mut receiver: SplitStream<WebSocket>) -> Result<(), Error> {
    while let Some(message) = receiver.next().await {
        let message = message?;
        match message {
            Message::Text(text) => {}
            Message::Binary(binary) => {}
            Message::Close(frame) => {}
            _ => continue,
        }
    }

    Ok(())
}

//本当はビデオエンコーディングして送信，frontendでVideoDecoderを使ってデコードとしたかったけど, Safari on iosではexperimentalだったので 画像をそのまま送信とする．
async fn sending(mut sender: SplitSink<WebSocket, Message>) -> Result<(), Error> {
    loop {
        let elapsed = APP.get().unwrap().start_time.elapsed().unwrap().as_secs() % 4;
        let mut image = File::open(format!("assets/{}.png", elapsed)).unwrap();
        let mut buffer = Vec::new();
        image.read_to_end(&mut buffer).unwrap();
        sender.send(Message::Binary(buffer)).await?;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
