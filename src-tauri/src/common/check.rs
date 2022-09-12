use std::fs;
use std::path::Path;

use tracing::info;

use crate::sql::{init_tag, init_code};

use super::config::Conf;

pub async fn check() {
    let path = Conf::gobal().path.as_str();
    check_dir(path);
    check_dir_db(path);
    check_table_tag(path).await;
    check_tabke_code(path).await;
}

fn check_dir(path: &str) {
    match Path::new(path).exists() {
        true => {
            info!("数据目录正常: {}", path);
        }
        false => {
            fs::create_dir(path).expect("创建数据目录失败");
            info!("数据目录创建成功: {}", path);
        }
    }
}

fn check_dir_db(path: &str) {
    // TODO 后续将地址固定化到配置类中， 由仓库地址自动产生
    let path = format!("{}{}", path, "db");
    match Path::new(path.as_str()).exists() {
        true => {
            info!("数据库目录正常: {}", path);
        }
        false => {
            fs::create_dir(path.clone()).expect("创建数据库目录失败");
            info!("数据库目录创建成功: {}", path);
        }
    }
}

async fn check_table_tag(path: &str) {
    let path = format!("{}{}", path, "db/tag.db");
    match Path::new(path.as_str()).exists() {
        true => {
            info!("tag表正常: {}", path);
        }
        false => {
            let rb = init_tag();
            rb.exec(
                "CREATE TABLE IF NOT EXISTS tag (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT ,
                `value` TEXT NOT NULL ,
                `flag` INTEGER default false);",
                vec![]).await.expect("创建tag表失败");
            info!("tag表创建成功: {}", path);
        }
    }
}

async fn check_tabke_code(path: &str) {
    let path = format!("{}{}", path, "db/code.db");
    match Path::new(path.as_str()).exists() {
        true => {
            info!("code表正常: {}", path);
        }
        false => {
            let rb = init_code();
            rb.exec(
                "CREATE TABLE IF NOT EXISTS code (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT ,
                `path` TEXT NOT NULL ,
                `desc` TEXT NOT NULL ,
                `lan`  TEXT NOT NULL ,
                `tags` TEXT NOT NULL ,
                `create_time` INTEGER NOT NULL,
                `update_time` INTEGER NOT NULL);",
                vec![]).await.expect("创建code表失败");
            info!("code表创建成功: {}", path);
        }
    }
}