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
use tokio::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub id: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub main: String,
}

impl Manifest {
    /// マニフェストファイルの読み込みと解析を行います
    /// # Example
    /// ```
    /// let manifest = Manifest::get().await;
    /// println!("plugin id: {}", manifest.id);
    /// ```
    pub async fn get() -> Manifest {
        let manifest_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("manifest.json");
        let manifest_str = read_to_string(manifest_path).await.unwrap();
        serde_json::from_str(&manifest_str).unwrap()
    }
}
