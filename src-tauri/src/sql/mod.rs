use std::fs;
use std::path::Path;

use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;
use tracing::info;

pub mod code;
pub mod tag;


pub fn init_sqlite(path: &str) -> Rbatis {
    let rb = Rbatis::new();
    rb.init(
        SqliteDriver {},
        &format!("sqlite://{}", path),
    )
        .unwrap();
    rb
}
