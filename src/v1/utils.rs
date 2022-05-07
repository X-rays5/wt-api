use worker::*;
use worker::{Result, Error};
use serde_json::*;

use crate::utils::*;

pub async fn parse_categories(ctx: &RouteContext<()>, country: &str, category: &str, check_exists: bool) -> Result<Vec<String>> {
    let mut categories: Vec<String> = Vec::new();
    for category in category.split(",").into_iter() {
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
        if is_category(category) {
            categories.push(category.into());
        } else {
            return Err(Error::from(format!("{} is a unknown category", category)));
        }
    }

    Ok(categories)
}

pub fn error_response(code: u16, message: &str) -> Result<Response> {
    let res = json!({
        "error": {
            "code": code,
            "message": message
        }
    });
    console_log!("Returning error code: {}\nWith message: {}", code, message);
    Response::error(res.to_string(), code)
}