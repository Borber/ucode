use rbatis::crud;
use serde::{Deserialize, Serialize};

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
