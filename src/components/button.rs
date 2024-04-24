use leptos::*;
use leptos_router::A;

#[component]
pub fn LinkBtn(
    #[prop(into)] href: String,
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    view! {
        <A href
            class=class + " tw-text-center tw-text-sm tw-font-bold tw-text-neutral-800 tw-border-solid tw-border-2 tw-rounded-xl tw-max-w-fit tw-px-4 tw-py-2 tw-bg-white dark:tw-text-white dark:tw-bg-graphite tw-transition tw-duration-200 hover:tw-text-greenLime hover:tw-border-greenLime"
        >
            {children()}
        </A>
    }
}
