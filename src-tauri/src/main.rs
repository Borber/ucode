#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[macro_use]
extern crate rbatis;
extern crate tantivy;

use anyhow::Result;
use tauri::command;
use tracing::info;

use crate::constants::LAN;

mod sql;
mod model;
mod constants;

#[command]
async fn greet(name: &str) -> Result<String, ()> {
    info!("类型: {}", serde_json::to_string(LAN.as_slice()).expect("获取类型失败"));
    Ok(format!("目前支持的语言类型共有: {}种 \n {}", LAN.len(), serde_json::to_string(LAN.as_slice()).expect("获取类型失败")))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
