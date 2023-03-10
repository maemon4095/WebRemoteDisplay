use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

pub fn configure_route(router: Router) -> Router {
    router.route("/", get(root))
}

async fn root() -> &'static str {
    "a"
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

async fn receiving(receiver: SplitStream<WebSocket>) {}

async fn sending(sender: SplitSink<WebSocket, Message>) {}
