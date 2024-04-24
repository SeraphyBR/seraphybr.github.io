use leptos::*;
use leptos_router::{use_params, Params, Redirect, A};

use crate::{
    components::{button::LinkBtn, content::BaseContent},
    contexts::posts::use_posts,
    models::posts::{PostData, PostMetadata},
    pages::base::BasePage,
};

#[component]
pub fn ProjectsListPage() -> impl IntoView {
    let posts = use_posts();

    view! {
        <BasePage title="Projetos">
            <div class="tw-vflex tw-justify-center tw-items-center tw-gap-5 tw-text-neutral-800 dark:tw-text-white tw-p-8">
                <div class="tw-vflex tw-items-center tw-gap-6 tw-pb-12">
                    <h1 class="tw-text-3xl tw-font-bold">Projetos</h1>
                    <LinkBtn href="/"><i class="fa fa-home"></i></LinkBtn>
                </div>
                <div class="tw-grid tw-grid-cols-1 md:tw-grid-cols-2 lg:tw-grid-cols-3 tw-gap-4">
                    {
                        posts.into_iter()
                            .filter(|p| p.metadata.project)
                            .map(|p| view! { <ProjectItem path=p.path metadata=p.metadata /> })
                            .collect_view()
                    }
                </div>
            </div>
        </BasePage>
    }
}

#[component]
fn ProjectItem(path: String, metadata: PostMetadata) -> impl IntoView {
    view! {
        <div class="tw-max-w-sm tw-rounded-xl tw-overflow-hidden tw-shadow-lg">
            <img
                class="tw-w-full tw-h-36"
                src=metadata.front_image
                alt=""
                style:background-color=metadata.front_color
            />
            <A href=path class="tw-text-gray-700 dark:tw-text-white hover:tw-text-greenLime">
                <div class="tw-px-6 tw-py-4">
                    <div class="tw-font-bold tw-text-xl tw-mb-2">{metadata.title}</div>
                    <p class="tw-text-base">
                        {metadata.brief}
                    </p>
                </div>
            </A>
        </div>
    }
}

#[derive(Params, PartialEq, Clone)]
struct ProjectPageUrlParams {
    path: String,
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params::<ProjectPageUrlParams>();

    move || {
        let path = params.get().map(|p| p.path).unwrap_or_default();

        let posts = use_posts();

        let project = posts
            .into_iter()
            .filter(|p| p.metadata.project)
            .find(|p| p.path == path);

        project
            .map(|p| view! { <ProjectContentPage post=p/> })
            .or_else(|| Some(view! { <Redirect path="/404"/> }))
            .into_view()
    }
}

#[component]
fn ProjectContentPage(post: PostData) -> impl IntoView {
    let bg_img = post.metadata.front_image.map(|bg| format!("url({bg})"));

    view! {
        <BasePage title=post.metadata.title.clone() enable_back_to_top=true>
            <BaseContent
                title=post.metadata.title
                bg_color=post.metadata.front_color
                bg_img=bg_img
                created_date=post.metadata.date
                inner_html=post.content
                back_href="/projects"
            >
            </BaseContent>
        </BasePage>
    }
}
