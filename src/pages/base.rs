use leptos::*;
use leptos_meta::Title;

use crate::components::card::Card;

#[component]
pub fn BasePage(
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Title text=title />
        <div class="tw-flex tw-justify-center tw-items-center tw-mt-auto tw-mb-auto tw-pt-7 tw-pb-7">
            <Card class>
                {children()}
            </Card>
        </div>
    }
}
