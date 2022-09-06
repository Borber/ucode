use std::iter::Iterator;
use std::string::ToString;

use once_cell::sync::Lazy;

// TODO 后期提供统计数量进行排序/固定顺序/或其他顺序
pub static LAN: Lazy<Vec<String>> = Lazy::new(||
    vec!["c", "cpp", "java", "rust"]
        .iter()
        .map(|s| s.to_string())
        .collect()
);