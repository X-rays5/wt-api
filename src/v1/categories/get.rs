use std::collections::HashMap;
use worker::*;
use worker::Result;
use serde_json::*;

use crate::utils::*;
use crate::v1::utils::*;

pub async fn get_categories(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    json_response(json!(get_vehicle_categories()), 200)
}

pub async fn country_has_categories(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let country = match ctx.param("country") {
        Some(val) => val,
        None => return error_response(500, "Massing country parameter")
    };
    match is_country(&ctx, country).await {
        Ok(_) => {},
        Err(err) => return error_response(400, err.to_string().as_str())
    }

    let category = match ctx.param("category") {
        Some(val) => val,
        None => return error_response(400, "Missing category parameter")
    };
    let categories = match parse_categories(&ctx, country, category, false).await {
        Ok(val) => val,
        Err(err) => return error_response(400, err.to_string().as_str())
    };

    let mut has_categories: HashMap<String, bool>= Default::default();
    for category in categories {
        let has = match country_has_category(&ctx, country, category.as_str()).await {
            Ok(val) => val,
            Err(err) => return error_response(500, err.to_string().as_str())
        };

        has_categories.insert(category, has);
    }

    json_response(json!(has_categories), 200)
}

pub async fn which_categories_per_country(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let res =match get_categories_for_countries(&ctx).await {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    json_response(json!(res), 200)
}