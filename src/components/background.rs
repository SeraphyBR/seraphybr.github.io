use leptos::*;


#[component]
pub fn Background(children: Children) -> impl IntoView {

    view! {
        <div class="tw-bg-no-repeat tw-bg-cover tw-h-full">{children()}</div>
    }
}