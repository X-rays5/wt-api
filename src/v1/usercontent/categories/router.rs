use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Deserialize, Serialize};
use crate::v1::usercontent::api::api::get_feed;
use crate::v1::usercontent::api::types::{FeedOptions, FeedResult};
use crate::v1::utils::{error_response, json_response};
use serde_json::json;
use worker::{Request, Response, Result, RouteContext};
use crate::utils::{db_get, db_get_key, db_write_key, get_unix_ts, should_update};

#[derive(Serialize, Deserialize, Clone)]
struct CachedFeedResult {
    updated_at: u64,
    feed: FeedResult
}

fn get_db_key_from_feed_options(feed_options: &FeedOptions) -> String {
    BASE64_STANDARD.encode(serde_json::to_string(feed_options).unwrap().as_bytes())
}

async fn retrieve_from_cache(ctx: &RouteContext<()>, feed_options: &FeedOptions) -> Option<CachedFeedResult> {
    let db = db_get(&ctx).unwrap();
    match db_get_key(&db, get_db_key_from_feed_options(feed_options)).await {
        Some(value) => {
            let feed_result: CachedFeedResult = serde_json::from_str(&value).unwrap();
            // Check if not older than 1 hour
            if should_update(feed_result.updated_at, 3600000) {
                return None;
            }
            Some(feed_result)
        }
        None => {
            None
        }
    }
}

async fn save_to_cache(ctx: &RouteContext<()>, feed_options: &FeedOptions, feed: &FeedResult) -> Option<CachedFeedResult> {
    let db = db_get(&ctx).unwrap();
    let value = CachedFeedResult {
        feed: feed.clone(),
        updated_at: get_unix_ts()
    };

    let val_tmp = value.clone();
    match db_write_key(&db, get_db_key_from_feed_options(feed_options), json!(val_tmp).to_string().as_str()).await {
        true => {
            Some(value)
        }
        false => {
            None
        }
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

            match retrieve_from_cache(&ctx, &options).await {
                Some(feed) => {
                    return json_response(json!(feed), 200);
                }
                None => {}
            }

            match get_feed(&options).await {
                Ok(feed) => {
                    let res = match save_to_cache(&ctx, &options, &feed).await {
                        Some(feed) => feed,
                        None => return error_response(500, "Failed to save to cache")
                    };
                    json_response(json!(res), 200)
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