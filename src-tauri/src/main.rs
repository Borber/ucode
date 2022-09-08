#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[macro_use]
extern crate rbatis;
extern crate tantivy;

use anyhow::Result;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use tauri::command;
use tracing::{error, info};

use crate::common::check::check;
use crate::constants::LAN;
use crate::sql::tag::{init_tag, Tag};

mod sql;
mod model;
mod constants;
mod common;

#[command]
async fn lans() -> Result<Vec<String>, ()> {
    let lans = LAN.to_vec();
    info!("返回语言类型: {:?}", lans);
    Ok(lans)
}

#[command]
async fn add_tag(name: String) -> Result<i64, ()> {
    let mut rb = init_tag();
    info!("数据库链接成功");
    match Tag::insert(&mut rb, &Tag {
        id: None,
        name: Some(name),
        flag: Some(0),
    }).await {
        Ok(result) => {
            info!("标签插入成功: {}", result.last_insert_id);
            Ok(result.last_insert_id.as_i64().unwrap())
        }
        Err(_) => {
            error!("标签插入失败");
            Err(())
        }
    }
}

#[command]
async fn all_tag() -> Result<Vec<Tag>, ()> {
    let mut rb = init_tag();
    Ok(Tag::select_all(&mut rb).await.expect("获取所有tag失败"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // TODO 后续通过读取配置来获取地址
    // 开发时不能设置为开发目录下的文件, 否则会被
    check("./data").await;

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            lans,
            add_tag,
            all_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
