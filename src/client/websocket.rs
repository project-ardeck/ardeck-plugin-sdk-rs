pub enum WebsocketState {
    None,
    Connected,
    Disconnected,
    Reconnecting,
    Error,
}

type WebSocket = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct Websocket {
    state: WebsocketState,
    stream: WebSocket,
}

impl Websocket {
    pub async fn new(port: &str) -> Self {
        let stream = Self::connect(port).await;

        Websocket {
            state: WebsocketState::None,
            stream
        }
    }

    async fn connect(port: &str,) -> WebSocket {
        let (ws, _) = tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{port}")).await.unwrap();

        ws
    }
}