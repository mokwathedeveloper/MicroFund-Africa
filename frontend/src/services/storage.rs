use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "microfund_token";

pub fn set_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token);
}

pub fn get_token() -> Option<String> {
    LocalStorage::get(TOKEN_KEY).ok()
}

pub fn remove_token() {
    LocalStorage::delete(TOKEN_KEY);
}

pub fn set_cache<T: serde::Serialize>(key: &str, data: &T) {
    let _ = LocalStorage::set(key, data);
}

pub fn get_cache<T: for<'de> serde::Deserialize<'de>>(key: &str) -> Option<T> {
    LocalStorage::get(key).ok()
}
