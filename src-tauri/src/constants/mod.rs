use std::iter::Iterator;
use std::string::ToString;

use once_cell::sync::Lazy;

pub static LAN: Lazy<Vec<String>> = Lazy::new(||
    vec!["c", "java", "rust"]
        .iter()
        .map(|s| s.to_string())
        .collect()
);