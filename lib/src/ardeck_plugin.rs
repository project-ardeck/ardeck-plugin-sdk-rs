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

use crate::{logger::init_logger, manifest::Manifest, SwitchInfo};

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
type WebSocketEventHandler<T> = Vec<(&'static str, Box<dyn Fn(T) + 'static>)>;

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
    action_handler: WebSocketEventHandler<SwitchInfo>,
    message_handler: WebSocketEventHandler<PluginMessage>,
}

impl ArdeckPlugin {
    pub async fn new() -> Self {
        init_logger().await;

        log::info!("starting ardeck-plugin\n\tversion: {}", env!("CARGO_PKG_VERSION"));

        let env: Vec<String> = env::args().collect();
        let port = env[1].clone();

        let stream = Self::connect(&port).await;

        ArdeckPlugin {
            state: WebsocketState::None,
            stream,
            action_handler: Vec::new(),
            message_handler: Vec::new(),
        }
    }

    async fn connect(port: &str) -> WebSocket {
        log::debug!("connecting to ws://127.0.0.1:{port}");
        let (ws_stream, _) = connect_async(format!("ws://127.0.0.1:{port}"))
            .await
            .unwrap();
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
                    log::debug!("message: {msg}");
                    let msg: PluginMessage = serde_json::from_str(&msg).unwrap();
                    match msg {
                        // PluginMessageData::Hello { plugin_version, ardeck_plugin_web_socket_version, plugin_id } => {

                        // }
                        PluginMessage::Success {
                            ardeck_studio_version,
                            ardeck_studio_web_socket_version,
                        } => {
                            self.state = WebsocketState::Connected;

                            log::info!(
                                "Success: \n\tardeck-studio-version: {ardeck_studio_version}\n\tardeck-studio-web-socket-version: {ardeck_studio_web_socket_version}"
                            );
                        }
                        PluginMessage::Message {
                            message_id,
                            message,
                        } => {
                            log::info!("Message: {message_id} {message}");
                        }
                        PluginMessage::Action(action) => {
                            log::debug!(
                                "[action]:\n\taction-id: {}\n\taction-data: {:?}",
                                &action.target.action_id,
                                &action.switch
                            );

                            let action_id = action.clone().target.action_id;
                            self.action_handler_emit_all(action_id, action.switch).await;
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
    pub async fn add_action_handler<F: Fn(SwitchInfo) + 'static>(
        &mut self,
        event_id: &'static str,
        handler: F,
    ) {
        self.action_handler
            // .lock()
            // .await
            .push((event_id, Box::new(handler)));
    }

    /// Ardeck studio からメッセージを受信したときに実行される動作を記述します。
    /// # Example
    /// ```
    /// plugin.add_message_handler("log" /* "log" or "error" */, |message| {
    ///     ...
    /// }).await;
    /// ```
    /*pub*/ async fn add_message_handler<F: Fn(PluginMessage) + 'static>(
        &mut self,
        message_id: &'static str,
        handler: F,
    ) {
        self.message_handler
            // .lock()
            // .await
            .push((message_id, Box::new(handler)));
    }

    async fn action_handler_emit_all(&mut self, event_id: String, data: SwitchInfo) {
        log::debug!("# event_handler_emit_all[request: {}]", event_id);
        for handler in self.action_handler.iter() {
            log::debug!("\thandler: {}", handler.0);
            if handler.0 == event_id {
                handler.1(data.clone());
            }
        }
    }

    async fn message_handler_emit_all(&mut self, event_id: String, data: PluginMessage) {
        log::debug!("# message_handler_emit_all[request: {}]", event_id);
        for handler in self.message_handler.iter() {
            log::debug!("\thandler: {}", handler.0);
            if handler.0 == event_id {
                handler.1(data.clone());
            }
        }
    }

    async fn send_hello(&mut self) {
        let manifest = Manifest::get().await;

        let data = PluginMessage::Hello {
            ardeck_plugin_web_socket_version: String::from("0.0.1"),
            plugin_id: manifest.id,
            plugin_version: manifest.version,
        };

        self.send(&serde_json::to_string(&data).unwrap()).await;
    }

    /// ardeck-studioとの通信を開始します
    pub async fn start_listening(&mut self) {
        self.state = WebsocketState::Connecting;
        self.send_hello().await;
        self.listen().await;
    }
}
