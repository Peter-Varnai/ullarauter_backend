use std::{sync::RwLock,
        collections::HashMap};
use lazy_static::lazy_static;



lazy_static! {
    pub static ref SIDEBAR_LOCK: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
    // pub static ref SIDEBAR_LOCK: RwLock<HashMap<&'static str, &'static str>> = RwLock::new(HashMap::new());
}


lazy_static! {
    pub static ref BIO_EXHIBS_LOCK: RwLock<String> = RwLock::new(String::new());
}


lazy_static! {
    pub static ref FRONTPAGE_EXHIBS: RwLock<String> = RwLock::new(String::new());
}
