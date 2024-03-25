use leptos::*;
use leptos_router::A;

use crate::{
    components::button::LinkBtn, contexts::posts::use_posts, models::posts::PostMetadata,
    pages::base::BasePage,
};

#[component]
pub fn PostsPage() -> impl IntoView {
    let posts = use_posts();

    view! {
        <BasePage title="Todos os Posts">
            <div class="tw-vflex tw-justify-center tw-items-center tw-gap-5 tw-text-neutral-800 tw-p-8">
                <div>
                    <h1 class="tw-text-3xl tw-font-bold">Todas as postagens</h1>
                </div>
                <ul class="tw-vflex tw-gap-8">
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
        <li class="tw-vflex tw-gap-4">
            <A href=path.clone() class="tw-font-bold tw-text-xl hover:tw-text-greenLime">{metadata.title}</A>
            <p class="tw-font-light tw-text-base">{metadata.brief}</p>
            <LinkBtn href=path>Ler Mais</LinkBtn>
        </li>
    }
}
