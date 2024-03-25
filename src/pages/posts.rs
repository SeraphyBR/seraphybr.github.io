use leptos::*;
use leptos_router::A;

use crate::{contexts::posts::use_posts, models::posts::PostMetadata, pages::base::BasePage};

#[component]
pub fn PostsPage() -> impl IntoView {
    let posts = use_posts();

    view! {
        <BasePage title="Todos os Posts">
            <div class="tw-flex tw-flex-col tw-justify-center tw-items-center tw-gap-5 tw-text-neutral-800">
                <div>
                    <h1 class="tw-text-3xl tw-font-bold">Todas as postagens</h1>
                </div>
                <ul>
                    {
                        posts.into_iter()
                            .map(|p| view! { <PostItem path=p.path metadata=p.metadata /> })
                            .collect_view()
                    }
                </ul>
            </div>
        </BasePage>
    }
}

#[component]
fn PostItem(path: String, metadata: PostMetadata) -> impl IntoView {
    view! {
        <li>
            <A href=path>{metadata.title}</A>
            <p>{metadata.brief}</p>
        </li>
    }
}
