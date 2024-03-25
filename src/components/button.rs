use leptos::*;
use leptos_router::A;

#[component]
pub fn LinkBtn(href: String, children: Children) -> impl IntoView {
    view! {
        <A href
            class="tw-text-sm tw-font-bold tw-border-solid tw-border-2 tw-rounded-xl tw-max-w-fit tw-px-4 tw-py-2 hover:tw-text-greenLime"
        >
            {children()}
        </A>
    }
}
