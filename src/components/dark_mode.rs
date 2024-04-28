use leptos::*;
use leptos_meta::Html;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
pub fn DarkModeToggleBtn() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let toggle = move |_| {
        if mode() == ColorMode::Dark {
            set_mode(ColorMode::Light);
        } else {
            set_mode(ColorMode::Dark);
        }
    };

    let icon = move || {
        if mode() == ColorMode::Dark {
            "fa-solid fa-moon tw-w-[14px]"
        } else {
            "fa-solid fa-sun"
        }
    };

    view! {
        <Html class=move || if mode() == ColorMode::Dark {"tw-dark"} else {""} />
        <div class="tw-fixed tw-top-6 tw-right-6">
            <button on:click=toggle class="tw-btn-primary !tw-bg-opacity-80">
                <i class=icon />
            </button>
        </div>
    }
}
