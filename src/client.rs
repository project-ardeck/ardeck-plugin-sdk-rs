use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod env_args;
pub mod websocket;

// enum OPS {
//     Hello
// }

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[repr(i32)]
pub enum PluginOp {
    Hello,
    Success,
    Message,
    Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "op", content = "data")]
pub enum PluginMessageData {
    Hello {
        // OP0: Hello
        plugin_version: String,
        ardeck_plugin_web_socket_version: String,
        plugin_id: String,
    },
    Success {
        // OP1: Success
        ardeck_studio_version: String,
        ardeck_studio_web_socket_version: String,
    },
    Message {
        // OP2: Message
        message_id: String,
        message: String,
    },
    Action {
        // OP3: Action
        action_id: String,
        action_data: ActionMap,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SwitchType {
    Digital = 0,
    Analog = 1,
}

pub type SwitchId = u8;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionMap {
    pub switch_type: SwitchType,
    pub switch_id: SwitchId,
    pub plugin_id: String,
    pub action_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessage {
    pub op: PluginOp,
    pub data: PluginMessageData,
}
