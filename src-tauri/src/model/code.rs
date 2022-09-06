use serde::Deserialize;

/// 代码片段主体
#[derive(Clone, Debug, Deserialize)]
pub struct Code {
    // 用途描述
    pub desc: Option<String>,
    // 语言类型
    pub lan: Option<String>,
    // 标签
    pub tags: Option<Vec<i64>>,
    // 主体
    pub body: Option<String>,
}