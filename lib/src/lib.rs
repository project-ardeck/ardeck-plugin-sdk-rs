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

/// スイッチの情報
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

/// プラグインとardeck-studioのメッセージのやり取りの段階と、そのデータ
/// # Op0: Hello
/// プラグインがardeck-studioに接続した際に送信するメッセージです
/// プラグインと専用のプロトコルのバージョン情報、プラグインのIDを含んでいます。
///
/// # Op1: Success
/// ardeck-studioとの接続を確立したことを表します。
/// ardeck-studioとのプロトコルのバージョン情報を含んでいます。
///
/// # Op2: Message
/// メッセージを送信する場合に使用します。
/// メッセージのIDとメッセージを含んでいます。
/// ## message_id
/// - `log`: 実行ログ
/// - `error`: エラーログ
///
/// # Op3: Action
/// アクションを送信する場合に使用します。
/// アクションの情報を含んでいます。
///
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

/// スイッチの種類
/// -1: Unknown, 0: Digital, 1: Analog
/// 仕様上、プラグインにSwitchTypeがUnknownのデータは送信されないため、未実装
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
