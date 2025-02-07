use serde::{Deserialize, Serialize};

pub mod ardeck_plugin;
pub mod manifest;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PluginOp {
    Hello,
    Success,
    Message,
    Action,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchInfo {
    pub switch_type: SwitchType, // -1: Unknown, 0: Digital, 1: Analog
    pub switch_id: SwitchId,
    pub switch_state: u16,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionTarget {
    pub plugin_id: String,
    pub action_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    switch: SwitchInfo,
    target: ActionTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "op", content = "data")] // TODO: opが数字でなく文字列で変換されてしまう問題
pub enum PluginMessage {
    #[serde(rename = "0")]
    Hello {
        // OP0: Hello
        plugin_version: String,
        ardeck_plugin_web_socket_version: String,
        plugin_id: String,
    },
    #[serde(rename = "1")]
    Success {
        // OP1: Success
        ardeck_studio_version: String,
        ardeck_studio_web_socket_version: String,
    },
    #[serde(rename = "2")]
    Message {
        // OP2: Message
        message_id: String,
        message: String,
    },
    #[serde(rename = "3")]
    Action(Action),
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
pub struct PluginMessageContainer {
    pub op: PluginOp,
    pub data: PluginMessage,
}
