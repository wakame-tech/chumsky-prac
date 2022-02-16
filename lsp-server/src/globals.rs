use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub static TOKEN_TYPES: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
