use std::collections::HashMap;
use cfg_if::cfg_if;
use worker::*;
use worker::Result;
use scraper::*;
use serde_json::{json, Value};
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

pub fn string_name_format(s: &str) -> String {
    let mut str_copy = s.as_bytes().to_vec();
    str_copy = str_copy.to_ascii_lowercase();
    str_copy[0] = str_copy[0].to_ascii_uppercase();
    let str_copy = String::from_utf8(str_copy).unwrap();

    str_copy
}

pub fn get_unix_ts() -> u64 {
    Date::now().as_millis()
}

pub fn should_update(update_time: u64, cooldown: u64) -> bool {
    get_unix_ts() - update_time >= cooldown
}

pub async fn fetch_countries(db: &KvStore) -> Result<Vec<String>> {
    let mut res = match Fetch::Url("https://wiki.warthunder.com/Category:Aircraft_by_country".parse().unwrap()).send().await {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    if res.status_code() != 200 {
        return Err(Error::from("Received non 200 status code when getting countries"))
    }

    let body = match res.text().await {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    let mut countries: Vec<String> = Default::default();
    let body = Html::parse_document(body.as_str());
    let selector = Selector::parse("div.mw-category-group").unwrap();
    for div in body.select(&selector) {
        let selector = Selector::parse("a").unwrap();
        for country in div.select(&selector) {
            let country = country.inner_html();
            let country = country.split(" ").next().unwrap();

            countries.push(country.to_lowercase());
        }
    };

    let unix_time = get_unix_ts();
    let countries_json = json!({
        "updated_at": unix_time,
        "countries": countries
    });
    db_write_key(&db, "countries".into(), countries_json.to_string().as_str()).await;

    Ok(countries)
}

pub async fn get_countries(ctx: &RouteContext<()>) -> Result<Vec<String>> {
    let db = match db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    Ok(match db_get_key(&db, "countries".into()).await {
        Some(val) => {
            let json: Value = serde_json::from_str(&*val).unwrap();
            if should_update(json["updated_at"].as_u64().unwrap(), 86400000) {
                match fetch_countries(&db).await {
                    Ok(val) => val,
                    Err(err) => return Err(err)
                }
            } else {
                let countries = json["countries"].as_array().unwrap();
                let mut countries_tmp: Vec<String> = Default::default();
                for country in countries {
                    countries_tmp.push(country.as_str().unwrap().to_string())
                }

                countries_tmp
            }
        }
        None => {
            match fetch_countries(&db).await {
                Ok(val) => val,
                Err(err) => return Err(err)
            }
        }
    })
}

//noinspection DuplicatedCode
pub async fn is_country(ctx: &RouteContext<()>, country: &str) -> Result<bool> {
    match get_countries(ctx).await {
        Ok(val) => {
            let country_tmp = country.to_lowercase();
            for country in val {
                if country.to_lowercase() == country_tmp {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Err(err) => return Err(err)
    }
}

pub fn get_vehicle_categories() -> Vec<&'static str> {
    vec!["aircraft", "helicopters", "ground", "naval"]
}

//noinspection DuplicatedCode
pub fn is_category(category: &str) -> bool {
    let categories = get_vehicle_categories();
    let category_tmp = category.to_lowercase();
    for category in categories {
        if category.to_lowercase() == category_tmp {
            return true;
        }
    }
    false
}

pub async fn fetch_categories_for_countries(ctx: &RouteContext<()>, db: &KvStore) -> Result<HashMap<String, HashMap<String, bool>>> {
    let mut result: HashMap<String, HashMap<String, bool>> = Default::default();
    let countries = match get_countries(ctx).await {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    let categories = vec!["Aviation", "Helicopters", "Ground_vehicles", "Fleet"];
    for category in categories {
        let mut res = match Fetch::Url(format!("https://wiki.warthunder.com/{}", category).parse().unwrap()).send().await {
            Ok(val) => val,
            Err(err) => return Err(err)
        };

        if res.status_code() != 200 {
            return Err(Error::from("Received non 200 status code when getting categories_for_countries"));
        } else {
            let body = match res.text().await {
                Ok(val) => val.to_lowercase(),
                Err(err) => return Err(err)
            };

            for country in &countries {
                let country_tmp = country.to_lowercase();

                if !result.contains_key(country_tmp.as_str()) {
                    result.insert(country_tmp.clone(), Default::default());
                }
                let category = match category {
                    "Aviation" => "aircraft",
                    "Helicopters" => "helicopters",
                    "Ground_vehicles" => "ground",
                    "Fleet" => "naval",
                    _ => unreachable!()
                };

                result.get_mut(country_tmp.as_str()).unwrap().insert(category.into(), body.contains(country_tmp.as_str()));
            }
        }
    }

    let unix_time = get_unix_ts();
    let result_json = json!({
        "updated_at": unix_time,
        "countries": result
    });
    db_write_key(db, "countries_have".into(), result_json.to_string().as_str()).await;

    Ok(result)
}

pub async fn get_categories_for_countries(ctx: &RouteContext<()>) -> Result<HashMap<String, HashMap<String, bool>>> {
    let db = match db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    Ok(match db_get_key(&db, "countries_have".into()).await {
        Some(val) => {
            let json: Value = serde_json::from_str(&*val).unwrap();
            if should_update(json["updated_at"].as_u64().unwrap(), 86400000) {
                match fetch_categories_for_countries(&ctx, &db).await {
                    Ok(val) => val,
                    Err(err) => return Err(err)
                }
            } else {
                let mut result: HashMap<String, HashMap<String, bool>> = Default::default();
                let countries = json["countries"].as_object().unwrap();
                for country in countries {
                    let mut has_category: HashMap<String, bool> = Default::default();
                    for has in country.1.as_object().unwrap() {
                        has_category.insert(has.0.to_string(), has.1.as_bool().unwrap());
                    }
                    result.insert(country.0.to_string(), has_category);
                }

                result
            }
        }
        None => {
            match fetch_categories_for_countries(ctx, &db).await {
                Ok(val) => val,
                Err(err) => return Err(err)
            }
        }
    })
}

pub async fn country_has_category(ctx: &RouteContext<()>, country: &str, category: &str) -> Result<bool> {
    let country = country.to_lowercase();
    let category = category.to_lowercase();
    match get_categories_for_countries(ctx).await {
        Ok(val) => {
            Ok(val[&country][&category])
        }
        Err(err) => return Err(err)
    }
}

const DEBUG_KV: bool = false;

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