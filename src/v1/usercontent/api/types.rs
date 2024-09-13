use serde::{Serialize, Deserialize};
use strum_macros::IntoStaticStr;

// Feed request
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, IntoStaticStr)]
pub enum ContentType {
    all,
    image,
    video,
    camouflage,
    sight,
    mission,
    location,
    model,
    sound,
    controls
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, IntoStaticStr)]
pub enum ContentSorting {
    created, // Recent
    rating, // Popular
    comments, // Most commented
    downloads, // Most downloaded
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FeedOptions {
    pub content: ContentType,
    pub sort: ContentSorting,
    pub user: Option<String>, // Author
    pub period: Option<i32>, // Days before now
    pub searchString: Option<String>, // Array of # searches separated by spaces
    pub page: i32, // Page id
    pub featured: Option<i32>, // No clue what this does
    pub subtype: Option<String> // No clue what this does
}


// Feed request response
#[derive(Serialize, Deserialize)]
pub struct FeedResult {
    pub status: String,
    pub data: FeedData
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FeedData {
    pub list: Vec<FeedItem>,
    pub pageTitle: String, // HTML page title
    pub link: String, // Relative URL browser
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FeedItem {
    pub lang_group: i32,
    pub id: i32,
    pub language: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub created: i64,
    pub visible: bool,
    pub isSpecial: bool,
    pub author: FeedItemAuthor,
    pub likes: i32,
    pub views: i32,
    pub doubt: bool,
    pub featured: bool,
    pub downloads: i32,
    pub comments: i32,
    pub isPinned: bool,
    pub isMarketSuitable: bool,
    pub description: String,
    pub images: Vec<FeedItemImage>,
    pub video_info: Option<FeedItemVideo>, // Videos only
    pub file: Option<FeedItemFile>,
    pub pbr_ready: Option<bool>,
    pub inverted_roughness: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FeedItemAuthor {
    pub id: i32,
    pub nickname: String,
    pub avatar: String,
}

#[derive(Serialize, Deserialize)]
pub struct FeedItemImage {
    pub id: i32,
    #[serde(rename = "type")]
    pub content_type: String,
    pub src: String,
    pub width: i32,
    pub height: i32,
    pub ratio: f32,
}

#[derive(Serialize, Deserialize)]
pub struct FeedItemVideo {
    #[serde(rename = "type")]
    pub src: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct FeedItemFile {
    pub id: i32,
    pub name: String,
    pub link: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub size: i32,
}