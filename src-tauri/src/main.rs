#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use anyhow::{Result};
use common::config::Conf;
use model::code::CodeDTO;
use rbatis::rbdc::datetime::FastDateTime;
use sql::code::Code;
use sql::init_code;
use tauri::command;
use tracing::{error, info};
use itertools::Itertools;

use crate::common::check::check;
use crate::constants::LAN;
use crate::sql::init_tag;
use crate::sql::tag::Tag;

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

#[command]
async fn add_code(code: CodeDTO) -> Result<bool, ()> {
    let mut rb = init_code();
    let date = FastDateTime::now().unix_timestamp();
    let tags = code.tags.unwrap().iter().join(",");
    
    let code = Code { id: None, path: Some("".to_string()), desc: code.desc, lan: code.lan, tags: Some(tags), create_time: Some(date), update_time: Some(date) };
    Code::insert(&mut rb, &code).await.unwrap();
    info!("片段添加成功");
    Ok(true)
}

#[tokio::main]
async fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    // 初始化全局配置
    Conf::init_conf();
    // 检查基础设备
    check().await;
    // 设置`tauri`的异步运行时
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            lans,
            add_tag,
            all_tag,
            add_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
