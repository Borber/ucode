use rbatis::{crud};
use serde::{Deserialize, Serialize};

/// TODO 多个表同时管理文章 ? 如果一个表被占用可以打开新的来查询相关
/// TODO 读写分离
/// TODO 文章历史表

/// 代码片段
/// path: 分库地址, 用以提升插入速度
/// desc: 描述代码作用
/// lan: 编程语言类型
/// tags: 标签列表, 使用String储存所有tag_id
/// creat_time: 创建时间
/// update_time: 更新时间
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Code {
    pub id: Option<i64>,
    pub path: Option<String>,
    pub desc: Option<String>,
    pub lan: Option<i64>,
    pub tags: Option<String>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
}

crud!(Code {});