use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;
use tracing::info;

use crate::common::config::Conf;

pub mod code;
pub mod tag;

pub fn init_code() -> Rbatis {
    init_sqlite("code.db")
}

pub fn init_tag() -> Rbatis {
    init_sqlite("tag.db")
}

fn init_sqlite(path: &str) -> Rbatis {
    let path = format!("{}{}{}", Conf::gobal().path, "db/", path);
    let rb = Rbatis::new();
    rb.init(
        SqliteDriver {},
        &format!("sqlite://{}", path),
    )
        .unwrap();
    info!("初始化连接成功: {}", path);
    rb
}
