use std::collections::HashMap;
use worker::*;
use serde_json::{json, Value};

use crate::v1::vehicles::valid_args::*;
use crate::v1::vehicles::update::*;
use crate::utils;

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

    let db = match utils::db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let mut res: HashMap<String, Value> = Default::default();
    for category in categories {
        let mut updated: bool = false;
        let mut category_json = match utils::db_get_key(&db, format!("{}_{}", country.to_lowercase(), category.to_lowercase())).await {
            Some(val) => {Value::from(val.as_str())}
            None => {updated = true; update_vehicles(country.to_lowercase().as_str(), category.to_lowercase().as_str()).await}
        };
        let updated_at: u64 = match category_json.get("updated_at") {
            Some(val) => {
                match val.as_u64() {
                    Some(val) => val,
                    None => return error_response(500, "Failed to get updated_at as u64")
                }
            }
            None => return error_response(500, "Failed to get updated_at")
        };
        let current_ts = get_unix_ts();
        if updated || current_ts - updated_at >= 86400000 {
            if !updated {
                category_json = update_vehicles(country, "ground").await;
            }
            match category_json.get("error") {
                None => {utils::db_write_key(&db, format!("{}_{}", country.to_lowercase(), category), category_json.to_string().as_str()).await;}
                Some(_) => {}
            }
        }
        res.insert(category.to_lowercase(), category_json);
    }

    Response::ok(json!(res).to_string())
}

async fn country_all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let country = ctx.param("country").unwrap();

    let db = match utils::db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let categories= vec!["ground", "helicopters", "planes", "naval"];
    let mut res: HashMap<String, Value> = Default::default();
    for category in categories {
        let mut category_json = match utils::db_get_key(&db, format!("{}_{}", country.to_lowercase(), category)).await {
            Some(val) => Value::from(val.as_str()),
            None => update_vehicles(country, category).await
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
        let current_ts = get_unix_ts();
        console_log!("{}", format!("{}_{}", country.to_lowercase(), category).as_str());
        if current_ts - updated_at >= 86400000 {
            category_json = update_vehicles(country, "ground").await;
            match category_json.get("error") {
                None => {utils::db_write_key(&db, format!("{}_{}", country.to_lowercase(), category), category_json.to_string().as_str()).await;}
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