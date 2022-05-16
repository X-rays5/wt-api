use std::collections::HashMap;
use worker::*;
use worker::Result;
use serde_json::*;

use crate::utils::*;
use crate::v1::utils::*;

pub async fn countries(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = match db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let result: Value = match db_get_key(&db, "countries".into()).await {
        Some(val) => from_str(val.as_str()).unwrap(),
        None => {
            match fetch_countries(&db).await {
                Ok(_) => {},
                Err(err) => return error_response(500, err.to_string().as_str())
            };

            return json_response(serde_json::from_str(db_get_key(&db, "countries".into()).await.unwrap().as_str()).unwrap(), 200)
        }
    };
    if should_update(result["updated_at"].as_u64().unwrap(), 86400000) {
        match fetch_countries(&db).await {
            Ok(_) => {},
            Err(err) => return error_response(500, err.to_string().as_str())
        };

        return json_response(serde_json::from_str(db_get_key(&db, "countries".into()).await.unwrap().as_str()).unwrap(), 200)
    } else {
        return json_response(result, 200)
    }
}

pub async fn have_category(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let category = match ctx.param("category") {
        Some(val) => val,
        None => return error_response(400, "Missing category parameter")
    };
    if !is_category(category) {
        return error_response(400, format!("{} is a unknown category", category).as_str())
    }

    let countries = match get_countries(&ctx).await {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let mut have_category: HashMap<String, bool> = Default::default();
    for country in countries {
        match country_has_category(&ctx, country.as_str(), category).await {
            Ok(val) => {have_category.insert(country, val);},
            Err(err) => return error_response(500, err.to_string().as_str())
        }
    }

    json_response(json!(have_category), 200)
}