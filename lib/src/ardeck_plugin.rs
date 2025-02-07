/*
Copyright (C) 2025 Project Ardeck

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{env, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
};

use super::{Action, PluginMessage, PluginMessageContainer, PluginOp};

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
type WebSocketEventHandler<T> = Arc<Mutex<Vec<(&'static str, Box<dyn Fn(T) + 'static>)>>>;

/// # Example
/// ```
/// let mut plugin = ArdeckPlugin::new().await;
/// 
/// plugin.add_action_handler("hello", |action| {
///     println!("Hello Ardeck!");
/// }).await;
/// ```
pub struct ArdeckPlugin {
    state: WebsocketState,
    stream: WebSocket,
    action_handler: WebSocketEventHandler<Action>,
    message_handler: WebSocketEventHandler<PluginMessage>,
}

impl ArdeckPlugin {
    pub async fn new() -> Self {
        let env: Vec<String> = env::args().collect();
        let port = env[1].clone();

        let stream = Self::connect(&port).await;

        tokio::spawn(async move {});

        ArdeckPlugin {
            state: WebsocketState::None,
            stream,
            action_handler: Arc::new(Mutex::new(Vec::new())),
            message_handler: Arc::new(Mutex::new(Vec::new())),
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
                    match msg {
                        // PluginMessageData::Hello { plugin_version, ardeck_plugin_web_socket_version, plugin_id } => {

                        // }
                        PluginMessage::Success {
                            ardeck_studio_version,
                            ardeck_studio_web_socket_version,
                        } => {
                            self.state = WebsocketState::Connected;

                            println!("[success]:\n\tardeck-studio-version: {ardeck_studio_version}\n\tardeck-studio-web-socket-version: {ardeck_studio_web_socket_version}");
                        }
                        PluginMessage::Message {
                            message_id,
                            message,
                        } => {
                            println!(
                                "[message]:\n\tmessage-id: {message_id}\n\tmessage: {message}"
                            );
                        }
                        PluginMessage::Action(action) => {
                            println!(
                                "[action]:\n\taction-id: {}\n\taction-data: {:?}",
                                &action.target.action_id, &action.switch
                            );

                            let action_id = action.clone().target.action_id;
                            self.action_handler_emit_all(
                                action_id,
                                action,
                            )
                            .await;
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

    /// アクションが発生した際に実行される動作を記述します。
    /// # Example
    /// ```
    /// plugin.add_action_handler("ACTION_ID", |action| {
    ///     ...
    /// }).await;
    /// ```
    pub async fn add_action_handler<F: Fn(Action) + 'static>(
        &mut self,
        event_id: &'static str,
        handler: F,
    ) {
        self.action_handler
            .lock()
            .await
            .push((event_id, Box::new(handler)));
    }

    pub async fn add_message_handler<F: Fn(PluginMessage) + 'static>(
        &mut self,
        event_id: &'static str,
        handler: F,
    ) {
        self.message_handler
            .lock()
            .await
            .push((event_id, Box::new(handler)));
    }

    async fn action_handler_emit_all(&mut self, event_id: String, data: Action) {
        println!("# event_handler_emit_all[request: {}]", event_id);
        for handler in self.action_handler.lock().await.iter() {
            println!("\thandler: {}", handler.0);
            if handler.0 == event_id {
                handler.1(data.clone());
            }
        }
        println!();
    }

    async fn message_handler_emit_all(&mut self, event_id: String, data: PluginMessage) {
        println!("# message_handler_emit_all[request: {}]", event_id);
        for handler in self.message_handler.lock().await.iter() {
            println!("\thandler: {}", handler.0);
            if handler.0 == event_id {
                handler.1(data.clone());
            }
        }
        println!();
    }

    async fn send_hello(&mut self) {
        // // TODO: manifestとactionsを読み込む関数を別にする
        // let str = String::new();
        // let manifest_file = File::open("manifest.json").unwrap();
        // let reader = std::io::BufReader::new(manifest_file);
        // let manifest: Manifest = serde_json::from_reader(reader).unwrap();
        

        let data = PluginMessage::Hello {
            ardeck_plugin_web_socket_version: String::from("0.0.1"),
            plugin_id: String::from("6ddf86cb-013b-4545-9ff0-854ca396ee6e"),
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
