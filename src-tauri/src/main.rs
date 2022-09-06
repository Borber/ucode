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
async fn lans() -> Result<Vec<String>, ()> {
    let lans = LAN.to_vec();
    info!("返回语言类型: {:?}", lans);
    Ok(lans)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![lans])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
