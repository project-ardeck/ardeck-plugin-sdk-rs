use std::{ascii, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
};

use super::{PluginMessage, PluginMessageData, PluginOp};

pub enum WebsocketState {
    None,
    Connecting,
    Connected,
    Disconnected,
    Reconnecting,
    Error,
}

type WebSocket =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type WebSocketEventHandler = Arc<Mutex<Vec<(&'static str, Box<dyn Fn(PluginMessage) + 'static>)>>>;

pub struct Websocket {
    state: WebsocketState,
    stream: WebSocket,
    listen_event_handler: WebSocketEventHandler,
}

impl Websocket {
    pub async fn new(port: &str) -> Self {
        let mut stream = Self::connect(port).await;

        tokio::spawn(async move {});

        Websocket {
            state: WebsocketState::None,
            stream,
            listen_event_handler: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn connect(port: &str) -> WebSocket {
        println!("connecting to ws://127.0.0.1:{port}");
        // let (ws, _) = tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{port}")).await.unwrap();
        let (mut ws_stream, _) = connect_async(format!("ws://127.0.0.1:{port}"))
            .await
            .unwrap();

        // let (mut sink, stream) = ws_stream.split();

        ws_stream
    }

    async fn send(&mut self, data: &str) {
        self.stream
            .send(Message::Text(Utf8Bytes::from(data)))
            .await
            .unwrap();
    }

    async fn listen(&mut self) {
        while let Some(msg) = self.stream.next().await {
            match msg {
                Ok(Message::Text(msg)) => {
                    println!("msg: {:?}", msg);
                    let msg: PluginMessage = serde_json::from_str(&msg).unwrap();
                    match msg.data {
                        // PluginMessageData::Hello { plugin_version, ardeck_plugin_web_socket_version, plugin_id } => {

                        // }
                        PluginMessageData::Success {
                            ardeck_studio_version,
                            ardeck_studio_web_socket_version,
                        } => {
                            self.state = WebsocketState::Connected;

                            println!("[success]:\n\tardeck-studio-version: {ardeck_studio_version}\n\tardeck-studio-web-socket-version: {ardeck_studio_web_socket_version}");
                        }
                        PluginMessageData::Message {
                            message_id,
                            message,
                        } => {
                            println!(
                                "[message]:\n\tmessage-id: {message_id}\n\tmessage: {message}"
                            );
                        }
                        PluginMessageData::Action(action) => {
                            println!(
                                "[action]:\n\taction-id: {}\n\taction-data: {:?}",
                                action.target.action_id, action.switch
                            );
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    println!("error: {e:?}");
                }
                _ => {}
            }
        }
    }

    pub async fn add_event_handler<F: Fn(PluginMessage) + 'static>(
        &mut self,
        event_name: &'static str,
        handler: F,
    ) {
        self.listen_event_handler
            .lock()
            .await
            .push((event_name, Box::new(handler)));
    }

    async fn send_hello(&mut self) {
        let data = PluginMessageData::Hello {
            ardeck_plugin_web_socket_version: String::from("0.0.1"),
            plugin_id: String::from("HELLO"),
            plugin_version: String::from("0.0.1"),
        };

        self.send(&serde_json::to_string(&data).unwrap()).await;
        println!("${:?}", &serde_json::to_string(&data).unwrap());
    }

    pub async fn start_listening(&mut self) {
        self.state = WebsocketState::Connecting;
        self.send_hello().await;
        self.listen().await;
    }
}
