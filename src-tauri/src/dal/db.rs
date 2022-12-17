use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::model::DbData;

pub static DB: Lazy<Mutex<HashMap<String, DbData>>> = Lazy::new(|| Mutex::new(HashMap::new()));
