use std::collections::HashMap;
use worker::*;
use serde_json::{json, Value};

use crate::v1::vehicles::valid_args::*;
use crate::v1::vehicles::update::*;

fn error_response(code: u16, message: &str) -> Result<Response> {
    let res = json!({
        "error": {
            "code": code,
            "message": message
        }
    });
    console_log!("Returning error code: {}\nWith message: {}", code, message);
    Response::error(res.to_string(), code)
}

pub async fn country_specific(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let country = match ctx.param("country") {
        Some(val) => {val}
        None => {return error_response(400, "Missing the country parameter")}
    };
    if country.to_lowercase() == "all" {
        return global_category(req, ctx).await
    }
    match ECountries::from_str(country) {
        Ok(_) => {}
        Err(_) => {return error_response(400, format!("{} is a unknown country", country).as_str())}
    };

    let category = match ctx.param("category") {
        Some(val) => {val}
        None => {return error_response(400, "Missing the category parameter")}
    };

    let mut categories: Vec<&str> = Vec::new();
    for category in category.split(",").into_iter() {
        if category.to_lowercase() == "all" {
            return country_all(req, ctx).await
        } else {
            categories.push(category);
        }
    };

    if categories.is_empty() {
        match EVehiclesCategories::from_str(category) {
            Ok(_) => {categories.push(category)}
            Err(_) => {return error_response(404, format!("{} is a unknown category", category).as_str())}
        }
    }

    let db = match ctx.kv("db") {
        Ok(val) => {val},
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let mut res: HashMap<String, Value> = Default::default();
    for category in categories {
        let mut category_json = match db.get(format!("{}_{}", country.to_lowercase(), category.to_lowercase()).as_str()).text().await {
            Ok(val) => {
                match val {
                    Some(val) => {Value::from(val.as_str())}
                    None => update_vehicles(country.to_lowercase().as_str(), category.to_lowercase().as_str()).await
                }
            }
            Err(err) => return error_response(500, err.to_string().as_str())
        };
        let updated_at: u64 = match category_json.get("updated_at") {
            Some(val) => {
                match val.as_str() {
                    Some(val) => {val.parse().unwrap()}
                    None => return error_response(500, "Failed to get updated_at as str")
                }
            }
            None => return error_response(500, "Failed to get updated_at")
        };
        let current_ts: u64 = match get_unix_ts().await {
            Ok(val) => val.parse().unwrap(),
            Err(_) => return error_response(500, "Failed to get unix timestamp")
        };
        if current_ts - updated_at >= 86400000 {
            category_json = update_vehicles(country, "ground").await;
            match category_json.get("error") {
                None => {db.put(format!("{}_{}", country.to_lowercase(), category).as_str(), category_json.to_string()).unwrap().execute().await.unwrap();}
                Some(_) => {}
            }
        }
        res.insert(category.to_lowercase(), category_json);
    }

    Response::ok(json!(res).to_string())
}

async fn country_all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let country = ctx.param("country").unwrap();

    let db = match ctx.kv("db") {
        Ok(val) => {val},
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let categories= vec!["ground", "helicopters", "planes", "naval"];
    let mut res: HashMap<String, Value> = Default::default();
    for category in categories {
        let mut category_json = match db.get(format!("{}_{}", country.to_lowercase(), category).as_str()).text().await {
            Ok(val) => {
                match val {
                    Some(val) => Value::from(val.as_str()),
                    None => update_vehicles(country, category).await
                }
            }
            Err(err) => return error_response(500, err.to_string().as_str())
        };
        let updated_at: u64;
        if category != "naval" {
             updated_at = match category_json.get("updated_at") {
                Some(val) => {
                    match val.as_str() {
                        Some(val) => {val.parse().unwrap()}
                        None => return error_response(500, "Failed to get updated_at as str")
                    }
                }
                None => return error_response(500, "Failed to get updated_at")
            };
        } else {
            updated_at = match category_json.get("coastal").unwrap().get("updated_at") {
                Some(val) => {
                    match val.as_str() {
                        Some(val) => {val.parse().unwrap()}
                        None => return error_response(500, "Failed to get updated_at as str")
                    }
                }
                None => return error_response(500, "Failed to get updated_at")
            };
        }
        let current_ts: u64 = match get_unix_ts().await {
            Ok(val) => val.parse().unwrap(),
            Err(_) => return error_response(500, "Failed to get unix timestamp")
        };
        if current_ts - updated_at >= 86400000 {
            category_json = update_vehicles(country, "ground").await;
            match category_json.get("error") {
                None => {db.put(format!("{}_{}", country.to_lowercase(), category).as_str(), category_json.to_string()).unwrap().execute().await.unwrap();}
                Some(_) => {}
            }
        }
        res.insert(category.parse().unwrap(), category_json);
    }

    Response::ok(json!(res).to_string())
}

pub async fn global_category(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    todo!()
}