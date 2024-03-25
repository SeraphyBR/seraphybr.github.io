use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,

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
