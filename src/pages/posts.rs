use leptos::*;
use serde::Deserialize;

use crate::pages::base::BasePage;

#[derive(Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,

    #[serde(default)]
    pub brief: String,

    #[serde(default)]
    pub project: bool,
}

#[derive(Debug)]
pub struct PostData {
    pub path: String,
    pub metadata: PostMetadata,
    pub content: String,
}

pub fn get_posts_data() -> Vec<PostData> {
    use crate::markdown_posts::FILES;
    use gray_matter::{engine::YAML, Matter};
    use markdown::{to_html_with_options, Options};

    let mut options = Options::default();

    options.parse.constructs.frontmatter = true;

    let posts = FILES
        .iter()
        .map(|file| {
            let matter = Matter::<YAML>::new();
            let path = file.0.to_owned();
            let markdown = matter.parse(file.1);

            let metadata: PostMetadata = markdown.data.unwrap().deserialize().unwrap();
            let content = to_html_with_options(file.1, &options).unwrap();

            PostData {
                path,
                metadata,
                content,
            }
        })
        .collect();

    posts
}

#[component]
pub fn PostsPage() -> impl IntoView {
    let posts = get_posts_data();
    view! {
        <BasePage title="Posts">
            <ul>
                {posts.into_iter().map(|p| view! { <li> {p.content} </li> }).collect_view()}
            </ul>
        </BasePage>
    }
}
