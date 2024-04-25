use crate::components::button::LinkBtn;
use leptos::*;
use leptos_meta::Html;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
pub fn DarkModeToggleBtn() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let on_click_toggle = move |_| {
        if mode() == ColorMode::Dark {
            set_mode(ColorMode::Light);
        } else {
            set_mode(ColorMode::Dark);
        }
    };

    view! {
        <Html class=move || if mode() == ColorMode::Dark {"tw-dark"} else {""} />
        <div class="tw-fixed tw-top-6 tw-right-6">
            <LinkBtn href="#" on:click=on_click_toggle class="!tw-bg-opacity-80">
                <i class="fa-solid fa-circle-half-stroke"/>
            </LinkBtn>
        </div>
    }
}
