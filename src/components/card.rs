use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="tw-w-2/4 tw-md:max-w-8xl tw-mx-auto tw-bg-white tw-text-gray-700 tw-p-8 tw-rounded-xl tw-shadow-md">
            {children()}
        </div>
    }
}
