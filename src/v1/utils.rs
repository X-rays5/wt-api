use reqwest::header::HeaderMap;
use worker::*;
use worker::{Result, Error};
use serde_json::*;

use crate::utils::*;

pub async fn parse_categories(ctx: &RouteContext<()>, country: &str, category: &str, check_exists: bool) -> Result<Vec<String>> {
    let mut categories: Vec<String> = Vec::new();
    let category = category.to_lowercase();
    for category in category.split(",").into_iter() {
        if category == "all" {
            let mut res: Vec<String> = Default::default();
            for string in get_vehicle_categories() {
                res.push(string.to_string());
            }

            return Ok(res);
        }

        if !is_category(category) {
            return Err(Error::from(format!("{} is a unknown category", category)));
        } else {
            if !check_exists || country_has_category(ctx, country, category).await.unwrap() {
                categories.push(category.into());
            } else {
                return Err(Error::from(format!("{} does not have the vehicle category: {}", country, category)));
            }
        }
    }

    if categories.is_empty() {
        if is_category(category.as_str()) {
            categories.push(category.into());
        } else {
            return Err(Error::from(format!("{} is a unknown category", category)));
        }
    }

    Ok(categories)
}

pub fn json_response(body: Value, status_code: u16) -> Result<Response> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
    headers.insert("Access-Control-Max-Age", "86400".parse().unwrap());

    let res = Response::from_json(&body).unwrap().with_headers(Headers::from(headers));
    Ok(res.with_status(status_code))
}

pub fn error_response(code: u16, message: &str) -> Result<Response> {
    let res = json!({
        "error": {
            "code": code,
            "message": message
        }
    });
    console_log!("Returning error code: {}\nWith message: {}", code, message);

    json_response(res, code)
}