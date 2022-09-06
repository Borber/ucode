use rbatis::{crud, impl_delete, impl_select, log};
use serde::{Deserialize, Serialize};

/// 标签
/// id: 主键
/// name: 标签名
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: Option<String>,
}

crud!(Tag {});