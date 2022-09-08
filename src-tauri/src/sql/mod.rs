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
    info!("初始化连接成功: {}", path);
    rb
}
