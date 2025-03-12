use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::{
  components::content::BaseContent,
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
          <h1 class="tw-text-3xl tw-font-bold tw-text-center">Projetos</h1>
          <A href="/" attr:class="tw-btn-primary"><i class="fa fa-home"></i></A>
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
      <div class="tw-max-w-sm tw-rounded-xl tw-overflow-hidden tw-shadow-lg dark:tw-shadow-none dark:tw-border-2 dark:tw-bg-graphite tw-border-clickable-colors">
          <img
              class="tw-w-full tw-h-36"
              src=metadata.front_image
              alt=""
              style:background-color=metadata.front_color.unwrap_or_default()
          />
          <A href=path attr:class="tw-text-clickable-colors">
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
  path: Option<String>,
}

#[component]
pub fn ProjectPage() -> impl IntoView {
  let params = use_params::<ProjectPageUrlParams>();

  move || {
    let path = params
      .get()
      .map(|p| p.path.unwrap_or_default())
      .unwrap_or_default();

    let posts = use_posts();

    let project = posts
      .into_iter()
      .filter(|p| p.metadata.project)
      .find(|p| p.path == path);

    project
      .map(|p| view! { <ProjectContentPage post=p/> }.into_any())
      .unwrap_or_else(|| view! { <Redirect path="/404"/> }.into_any())
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
