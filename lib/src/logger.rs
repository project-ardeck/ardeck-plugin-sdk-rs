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

use std::time::SystemTime;

use fern::colors::ColoredLevelConfig;
use tokio::fs::{self, File};

pub async fn init_logger() {
    if let Err(e) = init_logger_internal().await {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    };
}

async fn init_logger_internal() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_FILE: usize = 5;

    let log_dir = std::env::current_exe()?.parent().unwrap().join("logs");
    std::fs::create_dir_all(&log_dir)?;
    let log_file_name = format!("{}.log", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"));
    let log_file_path = log_dir.join(&log_file_name);
    File::create(&log_file_path).await?;
    delete_old_logs(MAX_FILE).await?;

    let colors = ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Blue)
        .debug(fern::colors::Color::White)
        .trace(fern::colors::Color::BrightBlack);

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_file_path)?)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ));
        })
        .apply();

    Ok(())
}

async fn delete_old_logs(max_file: usize) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = std::env::current_exe()?.parent().unwrap().join("logs");

    let mut files = std::fs::read_dir(log_dir)?
        .filter_map(|f| {
            f.ok().filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext == "log")
            })
        })
        .collect::<Vec<_>>();

    // タイムスタンプでソート（古い順）
    files.sort_by_key(|f| {
        f.metadata()
            .and_then(|m| m.created())
            .unwrap_or_else(|_| SystemTime::now())
    });
    files.reverse();

    for (i, d) in files.iter().enumerate() {
        if i >= max_file {
            fs::remove_file(d.path()).await?;
        }
    }

    Ok(())
}
