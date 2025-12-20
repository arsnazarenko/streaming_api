mod kafka;
mod models;

use axum::{
    Router,
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use futures_util::future;
use models::KafkaEvent;
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr};
use tokio::{sync::broadcast, task::JoinHandle};

const TOPICS: &[&str] = &[
    "dc_alerts",
    "dc_host_status",
    "dc_sla_compliance",
    "dc_zone_load",
    "dc_cpu_trend",
];

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<KafkaEvent>(100);

    // ---- Kafka task ----
    let mut kafka_tasks = vec![];
    for topic in TOPICS {
        let kafka_handle = tokio::spawn(kafka::run_consumer(*topic, tx.clone()));
        kafka_tasks.push(kafka_handle);
    }

    // ---- HTTP / WS ----
    let app = Router::new().route("/ws", get(ws_handler)).with_state(tx);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening on {}", addr);

    let server_handle: JoinHandle<()> = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::select! {
            res = future::join_all(kafka_tasks) => {
                panic!("Kafka task crashed: {:?}", res);
            }
            res = server_handle => {
                panic!("HTTP server task crashed: {:?}", res);
            }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(tx): State<broadcast::Sender<KafkaEvent>>,
) -> impl IntoResponse {
    let topic = match params.get("topic") {
        Some(t) => t.clone(),
        None => return "Missing topic parameter".into_response(),
    };

    let rx = tx.subscribe();

    ws.on_upgrade(move |socket| handle_socket(socket, topic, rx))
}

async fn handle_socket(
    mut socket: WebSocket,
    topic: String,
    mut rx: broadcast::Receiver<KafkaEvent>,
) {
    println!("Client connected, topic={}", topic);

    while let Ok(event) = rx.recv().await {
        if event.topic == topic {
            let msg = json!({
                "type": event.topic,
                "data": event.payload
            });

            if socket.send(Message::Text(msg.to_string())).await.is_err() {
                break;
            }
        }
    }

    println!("Client disconnected");
}
