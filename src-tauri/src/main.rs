#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use anyhow::{Result};
use common::config::Conf;
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
async fn add_tag(value: String) -> Result<i64, ()> {
    let mut rb = init_tag();
    info!("数据库链接成功");
    match Tag::insert(&mut rb, &Tag {
        id: None,
        value: Some(value),
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
    match Tag::select_all(&mut rb).await {
        Ok(tags) => {
            info!("获取所有tag成功");
            Ok(tags)
        },
        _ => {
            error!("获取所有tag失败");
            Ok(vec![]) // TODO 不知道这种写法是否合理， 但后期考虑使用统一返回体包装来处理状态
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    Conf::init_conf();

    check(Conf::gobal().path.as_str()).await;

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
