use chrono::{DateTime, Utc};
use leptos::*;

use leptos_router::A;

#[component]
pub fn BaseContent(
    #[prop(into)] title: String,
    #[prop(optional_no_strip)] bg_img: Option<String>,
    #[prop(optional_no_strip)] bg_color: Option<String>,
    #[prop(optional, into)] created_date: Option<DateTime<Utc>>,
    #[prop(into)] back_href: String,
    #[prop(optional, into)] inner_html: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="tw-vflex tw-justify-center tw-items-stretch tw-gap-5 tw-text-neutral-800 tw-p-8">
            <div class="tw-vflex tw-items-center tw-gap-6 tw-p-40 -tw-mt-16 -tw-ml-16 -tw-mr-16 tw-rounded-t-xl tw-bg-blend-multiply tw-bg-center tw-bg-cover"
                style:background-color=bg_color
                style:background-image=bg_img
            >
                <h1 class="tw-text-3xl tw-font-bold tw-text-center tw-text-white">{title}</h1>
                <Show when=move || created_date.is_some()>
                    <h4 class="tw-text-xl tw-font-bold tw-text-white">Criado em {created_date.map(|d| d.format("%d-%m-%Y").to_string())}</h4>
                </Show>
                <A href=back_href class="tw-btn-primary">
                    <i class="fa fa-chevron-left"></i>
                </A>
            </div>
            <article class="tw-prose dark:tw-prose-invert tw-max-w-none" inner_html=inner_html>
                {children.map(|c| c())}
            </article>
        </div>
    }
}
