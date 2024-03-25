use leptos::*;

use crate::{contexts::posts::use_posts, pages::base::BasePage};

#[component]
pub fn PostsPage() -> impl IntoView {
    let posts = use_posts();

    view! {
        <BasePage title="Posts">
            <ul>
                {posts.into_iter().map(|p| view! { <li> {p.content} </li> }).collect_view()}
            </ul>
        </BasePage>
    }
}
