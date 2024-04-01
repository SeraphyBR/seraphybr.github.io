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
            class=class + " tw-text-sm tw-font-bold tw-border-solid tw-border-2 tw-rounded-xl tw-max-w-fit tw-px-4 tw-py-2 tw-bg-white tw-transition tw-duration-200 hover:tw-text-greenLime"
        >
            {children()}
        </A>
    }
}
