use leptos::*;

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=class + " tw-w-3/4 tw-md:max-w-8xl tw-bg-white tw-text-gray-700 tw-p-8 tw-rounded-xl tw-shadow-md tw-max-w-4xl">
            {children()}
        </div>
    }
}
