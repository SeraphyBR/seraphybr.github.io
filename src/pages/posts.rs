use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use leptos_router::Redirect;
use leptos_router::A;

use crate::components::content::BaseContent;
use crate::models::posts::PostData;
use crate::{
    components::button::LinkBtn, contexts::posts::use_posts, models::posts::PostMetadata,
    pages::base::BasePage,
};

#[component]
pub fn PostListPage() -> impl IntoView {
    let posts = use_posts();

    view! {
        <BasePage title="Todos os Posts">
            <div class="tw-vflex tw-justify-center tw-items-center tw-gap-5 tw-text-neutral-800 tw-p-8">
                <div class="tw-vflex tw-items-center tw-gap-6 tw-pb-12">
                    <h1 class="tw-text-3xl tw-font-bold">Todas as postagens</h1>
                    <LinkBtn href="/"><i class="fa fa-home"></i></LinkBtn>
                </div>
                <ul class="tw-vflex tw-gap-8">
                    {
                        posts.into_iter()
                            .filter(|p| !p.metadata.project)
                            .map(|p| view! { <PostItem path=p.path metadata=p.metadata /> })
                            .collect_view()
                    }
                </ul>
            </div>
        </BasePage>
    }
}

#[derive(Params, PartialEq, Clone)]
struct PostPageUrlParams {
    path: String,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostPageUrlParams>();

    move || {
        let path = params.get().map(|p| p.path).unwrap_or_default();

        let posts = use_posts();

        let post = posts.into_iter().find(|p| p.path == path);

        post.map(|p| view! { <PostContentPage post=p/> })
            .or_else(|| Some(view! { <Redirect path="/404"/> }))
            .into_view()
    }
}

#[component]
fn PostContentPage(post: PostData) -> impl IntoView {
    let bg_img = post.metadata.front_image.map(|bg| format!("url({bg})"));

    view! {
        <BasePage title=post.metadata.title.clone()>
            <BaseContent
                title=post.metadata.title
                bg_color=post.metadata.front_color
                bg_img=bg_img
                created_date=post.metadata.date
                inner_html=post.content
                back_href="/posts"
            >
            </BaseContent>
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
