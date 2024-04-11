use leptos::*;

use crate::models::posts::{PostData, PostMetadata};

#[derive(Debug, Clone)]
pub struct PostContext(Vec<PostData>);

#[component]
pub fn PostsProvider(children: Children) -> impl IntoView {
    let posts = get_posts_data();

    provide_context(PostContext(posts));

    children()
}

pub fn use_posts() -> Vec<PostData> {
    use_context::<PostContext>().unwrap().0
}

fn get_posts_data() -> Vec<PostData> {
    use crate::markdown_posts::FILES;
    use gray_matter::{engine::YAML, Matter};
    use markdown::{to_html_with_options, Options};

    let mut options = Options::default();

    options.compile.allow_dangerous_html = true;
    options.parse.constructs.frontmatter = true;

    let mut posts: Vec<PostData> = FILES
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

    posts.sort_by_key(|p| p.metadata.date);
    posts.reverse();

    posts
}
