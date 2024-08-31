use crate::v1::usercontent::api::api::get_feed;
use crate::v1::usercontent::api::types::{ContentType, FeedOptions};
use crate::v1::utils::{error_response, json_response};
use serde_json::json;
use worker::{console_log, Request, Response, Result, RouteContext};

fn get_category(category: &String) -> ContentType {
    match category.as_str() {
        "camouflages" => ContentType::camouflage,
        "controls" => ContentType::controls,
        "images" => ContentType::image,
        "locations" => ContentType::location,
        "missions" => ContentType::mission,
        "models" => ContentType::model,
        "sights" => ContentType::sight,
        "sounds" => ContentType::sound,
        "videos" => ContentType::video,
        _ => { ContentType::all }
    }
}

pub async fn route_category(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match req.query::<FeedOptions>() {
        Ok(query) => {
            let mut options = query;
            match options.user {
                Some(_) => {}
                None => { options.user = Some("".to_string()) }
            }
            match options.period {
                Some(_) => {}
                None => { options.period = Some(i32::MAX) }
            }
            match options.searchString {
                Some(_) => {}
                None => { options.searchString = Some("".to_string()) }
            }
            options.featured = Some(0);
            options.subtype = Some("all".to_string());

            console_log!("Requesting feed for category body: {}", serde_json::to_string(&options).unwrap());
            match get_feed(options).await {
                Ok(feed) => {
                    json_response(json!(feed), 200)
                }
                Err(e) => {
                    error_response(500, &e.to_string())
                }
            }
        }
        Err(e) => {
            error_response(400, &e.to_string())
        }
    }
}