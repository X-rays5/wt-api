use std::collections::HashMap;
use worker::*;
use serde_json::{from_str, json, Value};
use worker::kv::KvStore;

use crate::v1::vehicles::update::*;
use crate::utils::*;
use crate::v1::utils::*;

async fn get_category(ctx: &RouteContext<()>, db: &KvStore, country: &str, category: &str) -> Result<Value> {
    if !country_has_category(ctx, country, category).await.unwrap() {
        return Err(Error::from(format!("{} does not have the vehicle category: {}", country, category)));
    }

    let mut updated: bool = false;
    let mut category_json: Value = match db_get_key(&db, format!("{}_{}", country.to_lowercase(), category.to_lowercase())).await {
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

    let current_ts = get_unix_ts();
    if updated || current_ts - updated_at >= 86400000 {
        if !updated {
            category_json = update_vehicles(country, category).await;
        }
        match category_json.get("error") {
            None => {db_write_key(&db, format!("{}_{}", country.to_lowercase(), category), category_json.to_string().as_str()).await;}
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

    if !is_country(&ctx, country).await.unwrap() {
        return error_response(404, format!("{} is a unknown country", country).as_str());
    }

    let category = match ctx.param("category") {
        Some(val) => {val}
        None => {return error_response(400, "Missing the category parameter")}
    };
    let category = category.to_lowercase();

    let categories = match parse_categories(&ctx, country, category.as_str(), true).await {
        Ok(val) => val,
        Err(err) => return error_response(404, err.to_string().as_str())
    };

    // check which categories are available
    let mut available_categories: Vec<String> = Vec::new();
    for cat in categories {
        if country_has_category(&ctx, country, cat.as_str()).await.unwrap() {
            available_categories.push(cat);
        }
    }

    let db = match db_get(&ctx) {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    let mut res: HashMap<String, Value> = Default::default();
    for category in available_categories {
        let vehicles = match get_category(&ctx, &db, country, category.as_str()).await {
            Ok(val) => val,
            Err(err) => return error_response(500, err.to_string().as_str())
        };
        res.insert(category.to_lowercase(), vehicles);
    }

    json_response(json!(res), 200)
}

pub async fn global_category(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let category = match ctx.param("category") {
        Some(val) => val,
        None => return error_response(400, "Missing category parameter")
    };
    let categories = match parse_categories(&ctx, "", category, true).await {
        Ok(val) => val,
        Err(err) => return error_response(400, err.to_string().as_str())
    };

    let mut res: HashMap<String, HashMap<String, Value>> = Default::default();
    let countries = match get_countries(&ctx).await {
        Ok(val) => val,
        Err(err) => return error_response(500, err.to_string().as_str())
    };

    for country in &countries {
      for category in &categories {
          if !res.contains_key(category) {
              res.insert(category.clone(), Default::default());
          }

          let has = match country_has_category(&ctx, country, category.as_str()).await {
              Ok(val) => val,
              Err(err) => return error_response(500, err.to_string().as_str())
          };
          if has {
              let db = match db_get(&ctx) {
                  Ok(val) => val,
                  Err(err) => return error_response(500, err.to_string().as_str())
              };

              let vehicles = match get_category(&ctx, &db, country, category.as_str()).await {
                  Ok(val) => val,
                  Err(err) => return error_response(500, err.to_string().as_str())
              };

              res.get_mut(category.as_str()).unwrap().insert(country.to_string(), vehicles);
          }
      }
    }

    json_response(json!(res), 200)
}