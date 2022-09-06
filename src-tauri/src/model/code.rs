use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Code {
    pub id: Option<i64>,
    pub desc: Option<String>,
    pub lan: Option<i64>,
    pub tags: Option<Vec<i64>>,
}