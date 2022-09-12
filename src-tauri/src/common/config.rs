use std::{fs::{File, self}, io::Read, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use tracing::{info, error};

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub path: String,
}

static DEFAULT_CONFIG_LOCATION: &str = "config.toml";
static DEFAULT_CONFIG: &str = include_str!("config.toml");
static G_CONF: OnceCell<Conf> = OnceCell::new();



impl Conf {
    pub fn gobal() -> &'static Conf {
        G_CONF.get().expect("配置未初始化")
    }

    pub fn init_conf() {
        let mut str_val = String::new();
        let config_str = match File::open(DEFAULT_CONFIG_LOCATION) {
            Ok(mut file) => {
                file.read_to_string(&mut str_val).expect("请确认配置文件编码格式");
                str_val.as_str()
            },
            _ => {
                info!("未找到配置文件");
                Self::create_default_conf();
                DEFAULT_CONFIG
            }};
        let conf: Conf = toml::from_str(config_str).expect("解析配置失败, 请查看你的配置文件");
        G_CONF.set(conf).expect("设置全局配置失败");
        info!("初始化全局配置成功");
    }

    pub fn create_default_conf() {
        match Path::new(DEFAULT_CONFIG_LOCATION).exists() {
            true => {
                error!("配置文件已存在， 请先删除");
            },
            false => {
                match fs::write(DEFAULT_CONFIG_LOCATION, DEFAULT_CONFIG) {
                    Ok(_) => {
                        info!("默认配置文件创建成功")
                    },
                    _ => {
                        error!("默认配置文件创建失败");
                    },
                };
            },
        };
    }

    pub fn delete_conf() {
        match fs::remove_file(DEFAULT_CONFIG_LOCATION) {
            Ok(_) => {
                info!("删除配置文件成功");
            },
            Err(_) => {
                error!("删除配置文件失败");
            },
        };
    }
}

