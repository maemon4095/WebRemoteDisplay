use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
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
        .route("/connection", get(connection))
}

async fn root() -> Html<&'static str> {
    Html(&*ROOT_PAGE)
}

async fn connection(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(socket)
}

async fn socket(socket: WebSocket) {
    let (sender, receiver) = socket.split();

    tokio::select! {
        v = receiving(receiver) => v,
        v = sending(sender) => v,
    };
}

async fn receiving(mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {}
            Message::Binary(binary) => {}
            Message::Close(frame) => {}
            _ => continue,
        }
    }
}

//本当はビデオエンコーディングして送信，frontendでVideoDecoderを使ってデコードとしたかったけど, Safari on iosではexperimentalだったので 画像をそのまま送信とする．
async fn sending(mut sender: SplitSink<WebSocket, Message>) {
    loop {
        let elapsed = APP.get().unwrap().start_time.elapsed().unwrap().as_secs() % 4;
        let mut image = File::open(format!("assets/{}.png", elapsed)).unwrap();
        let mut buffer = Vec::new();
        image.read_to_end(&mut buffer).unwrap();
        sender.send(Message::Binary(buffer)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
