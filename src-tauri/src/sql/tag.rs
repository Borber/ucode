use rbatis::{crud, Rbatis};
use serde::{Deserialize, Serialize};

use crate::sql::init_sqlite;

/// 标签
/// id: 主键
/// value: 标签内容
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub value: Option<String>,
    pub flag: Option<i64>,
}

crud!(Tag {});


/// TODO 后续通过配置文件获取地址, 而非写死
pub fn init_tag() -> Rbatis {
    init_sqlite("./data/db/tag.db")
}