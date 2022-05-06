use std::collections::HashMap;
use worker::*;
use serde_json::{from_str, json, Value};
use worker::kv::KvStore;

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

async fn get_category(db: &KvStore, country: &str, category: &str) -> Result<Value> {
    if !country_has_category(ECountries::from_str(country).unwrap(), EVehiclesCategories::from_str(category).unwrap()) {
        return Err(Error::from(format!("{} does noet have the vehicle category: {}", country, category)));
    }

    let mut updated: bool = false;
    let mut category_json: Value = match utils::db_get_key(&db, format!("{}_{}", country.to_lowercase(), category.to_lowercase())).await {
        Some(val) => from_str(&*val).unwrap(),
        None => {
            updated = true;
            update_vehicles(country.to_lowercase().as_str(), category.to_lowercase().as_str()).await
        }
    };

    let updated_at = match category_json["updated_at"].as_u64() {
        Some(val) => val,
        None => return Err(Error::from("Failed to get updated_at as u64"))
    };

    let current_ts = utils::get_unix_ts();
    if updated || current_ts - updated_at >= 86400000 {
        if !updated {
            category_json = update_vehicles(country, category).await;
        }
        match category_json.get("error") {
            None => {utils::db_write_key(&db, format!("{}_{}", country.to_lowercase(), category), category_json.to_string().as_str()).await;}
            Some(_) => {}
        }
    }

    Ok(category_json)
}

pub async fn country_specific(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let country = match ctx.param("country") {
        Some(val) => {val}
        None => {return error_response(400, "Missing the country parameter")}
    };
    if country.to_lowercase() == "all" {
        return global_category(req, ctx).await
    }

    if !is_country(country) {
        return error_response(404, format!("{} is a unknown country", country).as_str());
    }

    let category = match ctx.param("category") {
        Some(val) => {val}
        None => {return error_response(400, "Missing the category parameter")}
    };

    let mut categories: Vec<&str> = Vec::new();
    for category in category.split(",").into_iter() {
        if category.to_lowercase() == "all" {
            return country_all(req, ctx).await
        } else {
            if !is_category(category) {
                return error_response(404, format!("{} is a unknown category", category).as_str())
            } else {
                if country_has_category(ECountries::from_str(country).unwrap(), EVehiclesCategories::from_str(category).unwrap()) {
                    categories.push(category);
                } else {
                    return error_response(404, format!("{} does not have the vehicle category: {}", country, category).as_str());
                }
            }
        }
    };
    if categories.is_empty() {
        if is_category(category) {
            categories.push(category);
        } else {
            return error_response(404, format!("{} is a unknown category", category).as_str());
        }
    }

    let db = match utils::db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let mut res: HashMap<String, Value> = Default::default();
    for category in categories {
        let vehicles = match get_category(&db, country, category).await {
            Ok(val) => val,
            Err(err) => return error_response(500, err.to_string().as_str())
        };
        res.insert(category.to_lowercase(), vehicles);
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
        if country_has_category(ECountries::from_str(country).unwrap(), EVehiclesCategories::from_str(category).unwrap()) {
            let vehicles = match get_category(&db, country, category).await {
                Ok(val) => val,
                Err(err) => return error_response(500, err.to_string().as_str())
            };

            res.insert(category.parse().unwrap(), vehicles);
        }
    }

    Response::ok(json!(res).to_string())
}

pub async fn global_category(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    todo!()
}