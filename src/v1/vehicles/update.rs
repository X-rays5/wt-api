use serde::*;
use serde_json::*;
use worker::*;
use worker::Result;
use scraper::html::*;
use scraper::Selector;

use crate::utils::*;

const BASE_URL: &str = "https://wiki.warthunder.com/Category:";
const PLANE_URL: &str = "_aircraft";
const HELICOPTER_URL: &str = "_helicopters";
const GROUND_URL: &str = "_ground_vehicles";
const COASTAL_FLEET_URL: &str = "Coastal_Fleet_";
const BLUEWATER_FLEET_URL: &str = "Bluewater_Fleet_";

pub async fn update_vehicles(country: &str, category: &str) -> Value {
    let category = category.to_lowercase();
    let category = category.as_str();
    if !is_category(category) {
        return json!({"error": format!("Invalid category {}", category)})
    }

    let mut country = string_name_format(country);
    if country == "Ussr" || country == "Usa" {
        country = country.to_uppercase();
    }
    let country = country.as_str();

    let res = match category {
        "naval" => {
            let coastal = naval_vehicles(country, "coastal").await;
            if coastal.as_object().unwrap().contains_key("error") {
                return coastal;
            }

            let bluewater = naval_vehicles(country, "bluewater").await;
            if bluewater.as_object().unwrap().contains_key("error") {
                return bluewater;
            }

            let unix_time = get_unix_ts();
            return json!({
                "updated_at": unix_time,
                "coastal": coastal,
                "bluewater": bluewater
            });
        },
        _ => {
            let category = match category {
                "ground" => { GROUND_URL }
                "aircraft" => { PLANE_URL }
                "helicopters" => { HELICOPTER_URL }
                _ => { "" }
            };
            if category.is_empty() {
                return json!({
                "error": format!("Invalid category {}", category)
            })
            }

            parse_tree(Fetch::Url(format!("{}{}{}", BASE_URL, country, category).parse().unwrap()).send().await, country).await
        }
    };

    res
}

async fn naval_vehicles(country: &str, naval_type: &str) -> Value {
    match naval_type {
        "coastal" => {
            parse_tree(Fetch::Url(format!("{}{}{}", BASE_URL, COASTAL_FLEET_URL, country).parse().unwrap()).send().await, country).await
        },
        "bluewater" => {
            parse_tree(Fetch::Url(format!("{}{}{}", BASE_URL, BLUEWATER_FLEET_URL, country).parse().unwrap()).send().await, country).await
        }
        _ => { json!({"error": "Invalid naval vehicle category"}) }
    }
}

#[derive(Serialize, Deserialize)]
struct STreeItem {
    name: String,
    wiki_page: String,
    thumbnail_img_url: String,
}

async fn parse_tree(html_req: Result<Response>, country: &str) -> Value {
    let mut res: Response = match html_req {
        Ok(res) => {res}
        Err(err) => {return json!({"error": err.to_string()})}
    };

    if res.status_code() != 200 {
        return json!({
            "error": format!("received status code: {} while fetching vehicles", res.status_code())
        });
    }

    let html = res.text().await.unwrap();
    let html = Html::parse_document(html.as_str());
    let tree_item_selector = Selector::parse("div.tree-item").unwrap();

    let tree_item_selector_name = Selector::parse("div.tree-item-text").unwrap();
    let tree_item_selector_name_inner = Selector::parse("span").unwrap();
    let tree_item_selector_image = Selector::parse("div.tree-item-img").unwrap();
    let tree_item_selector_image_inner = Selector::parse("img").unwrap();
    let mut vehicles: Vec<STreeItem> = Vec::new();
    for tree_item in html.select(&tree_item_selector) {
        let mut vehicle: STreeItem = STreeItem { name: "".to_string(), wiki_page: "".to_string(), thumbnail_img_url: "".to_string()};
        vehicle.name = tree_item.select(&tree_item_selector_name).next().unwrap().select(&tree_item_selector_name_inner).next().unwrap().inner_html();
        vehicle.name = vehicle.name.replace("&nbsp;", "");
        vehicle.thumbnail_img_url = tree_item.select(&tree_item_selector_image).next().unwrap().select(&tree_item_selector_image_inner).next().unwrap().value().attr("src").unwrap().to_string();

        match vehicle.name.chars().next().unwrap().is_alphanumeric() {
            true => {}
            false => {
                vehicle.name.remove(0);
                vehicle.name += format!(" ({})", country.to_string()).as_str();
            }
        }
        let vehicle_name_url = vehicle.name.replace(" ", "_");
        vehicle.wiki_page = format!("https://wiki.warthunder.com/{}", vehicle_name_url);


        vehicles.push(vehicle);
    }

    let unix_time = get_unix_ts();

    return json!({
        "updated_at": unix_time,
        "vehicles": vehicles
    });
}