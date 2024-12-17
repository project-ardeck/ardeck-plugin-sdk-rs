use std::env;

mod client;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let env: Vec<String> = env::args().collect();
    println!("env: {env:?}");
    let port = env[1].clone();
    let ws = client::websocket::Websocket::new(&port).await;
}
