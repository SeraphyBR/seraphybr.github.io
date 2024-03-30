use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub date: DateTime<Utc>,

    pub front_image: Option<String>,
    pub front_color: Option<String>,

    #[serde(default)]
    pub brief: String,

    #[serde(default)]
    pub project: bool,
}

#[derive(Debug, Clone)]
pub struct PostData {
    pub path: String,
    pub metadata: PostMetadata,
    pub content: String,
}
