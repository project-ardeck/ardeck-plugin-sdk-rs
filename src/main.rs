use std::env;

use futures_util::SinkExt;
use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::{Message, Utf8Bytes}};

mod client;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let env: Vec<String> = env::args().collect();
    println!("env: {env:?}");
    let port = env[1].clone();
    
    let mut ws = client::websocket::Websocket::new(&port).await;

    ws.add_event_handler("hello", |msg| {
        println!("Hello!: {:?}", msg);
    }).await;

    ws.add_event_handler("ping", |msg| {
        println!("pong");
    }).await;

    ws.start_listening().await;
}
