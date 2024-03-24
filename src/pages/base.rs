use leptos::*;
use leptos_meta::Title;

use crate::components::card::Card;

#[component]
pub fn BasePage(#[prop(optional, into)] title: String, children: Children) -> impl IntoView {
    view! {
        <Title text=title />
        <div class="tw-flex tw-justify-center tw-items-center tw-h-[100vh]">
            <Card>
                {children()}
            </Card>
        </div>
    }
}