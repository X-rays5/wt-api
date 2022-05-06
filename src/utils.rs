use cfg_if::cfg_if;
use worker::*;
use crate::kv::{KvError, KvStore, ListResponse};

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

const DEBUG_KV: bool = true;

#[allow(dead_code)]
pub fn db_get(ctx: &RouteContext<()>) -> Result<KvStore> {
    ctx.kv("db")
}

#[allow(dead_code)]
pub async fn db_get_keys(db: &KvStore) -> std::result::Result<ListResponse, KvError> {
    db.list().execute().await
}

#[allow(dead_code)]
pub async fn db_get_key(db: &KvStore, key: String) -> Option<String> {
    if DEBUG_KV {
        console_log!("db_get_key: {}", key);
    }
    match db.get(key.as_str()).text().await {
        Ok(val) => val,
        Err(err) => {
            if DEBUG_KV {
                console_log!("KvError: {:?}", err);
            }
            Option::None
        }
    }
}

#[allow(dead_code)]
pub async fn db_write_key(db: &KvStore, key: String, value: &str) -> bool {
    if DEBUG_KV {
        console_log!("db_write: {} = {}", key, value);
    }
    match db.put(key.as_str(), value).unwrap().execute().await {
        Ok(_) => true,
        Err(err) => {
            if DEBUG_KV {
                console_log!("KvError: {:?}", err);
            }
            false
        }
    }
}

#[allow(dead_code)]
pub async fn db_delete_kv(db: &KvStore, key: String) -> bool {
    if DEBUG_KV {
        console_log!("db_delete: {}", key);
    }
    match db.delete(key.as_str()).await {
        Ok(_) => true,
        Err(err) => {
            if DEBUG_KV {
                console_log!("KvError: {:?}", err);
            }
            false
        }
    }
}