use leptos::*;

use crate::components::card::Card;

#[component]
pub fn BasePage(children: Children) -> impl IntoView {
    view! {
        <div class="tw-flex tw-justify-center tw-items-center tw-h-[100vh]">
            <Card>
                {children()}
            </Card>
        </div>
    }
}
