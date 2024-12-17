use serde::{Deserialize, Serialize};

pub mod websocket;
pub mod env_args;

pub enum PluginOp {
    Hello,
    Success,
    Message,
    Action
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessage {
    // OP0: Hello
    pub plugin_version: Option<String>,
    pub ardeck_plugin_web_socket_version: Option<String>,
    pub plugin_id: Option<String>,

    // OP1: Success
    pub ardeck_studio_version: Option<String>,
    pub ardeck_studio_web_socket_version: Option<String>,

    // OP2: Message
    pub message_id: Option<String>,
    pub message: Option<String>,

    // OP3: Action
    pub action_id: Option<String>,
    pub action_data: Option<ActionMap>
}

impl Default for PluginMessage {
    fn default() -> Self {
        PluginMessage {
            plugin_version: None,
            ardeck_plugin_web_socket_version: None,
            plugin_id: None,
            ardeck_studio_version: None,
            ardeck_studio_web_socket_version: None,
            message_id: None,
            message: None,
            action_id: None,
            action_data: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SwitchType {
    Digital = 0,
    Analog = 1
}

pub type SwitchId = u8;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionMap {
    pub switch_type: SwitchType,
    pub switch_id: SwitchId,
    pub plugin_id: String,
    pub action_id: String
}


