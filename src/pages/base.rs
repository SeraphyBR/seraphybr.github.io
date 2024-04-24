use leptos::*;
use leptos_meta::Title;

use crate::components::back_to_top::BackToTopBtn;
use crate::components::card::Card;
use crate::components::dark_mode_toggle::DarkModeToggleBtn;

#[component]
pub fn BasePage(
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] enable_back_to_top: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <Title text=title />
        <DarkModeToggleBtn />
        <div class="tw-flex tw-justify-center tw-items-center tw-mt-auto tw-mb-auto tw-pt-7 tw-pb-7">
            <Card class>
                {children()}
            </Card>
        </div>
        <Show when=move || enable_back_to_top>
            <BackToTopBtn/>
        </Show>
    }
}
