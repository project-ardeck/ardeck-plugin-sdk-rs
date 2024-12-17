pub enum WebsocketState {
    None,
    Connected,
    Disconnected,
    Reconnecting,
    Error,
}

pub struct Websocket {
    state: WebsocketState,
}

impl Websocket {
    pub async fn new(port: &str) -> Self {
        Self::connect(port).await;

        Websocket {
            state: WebsocketState::None,
        }
    }

    async fn connect(port: &str,) {
        
    }
}