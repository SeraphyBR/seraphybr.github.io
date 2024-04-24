use crate::components::button::LinkBtn;
use leptos::*;
use leptos_use::use_window_scroll;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn BackToTopBtn() -> impl IntoView {
    let (_, y) = use_window_scroll();

    let opacity_btn = move || {
        if y() > 300.0 {
            1
        } else {
            0
        }
    };

    let on_click_go_to_top = |_| {
        window().scroll_to_with_scroll_to_options(
            ScrollToOptions::new()
                .behavior(ScrollBehavior::Smooth)
                .top(0.0),
        );
    };

    view! {
        <div style:opacity=opacity_btn class="tw-fixed tw-bottom-6 tw-right-6 tw-transition-opacity tw-duration-700 tw-ease-in-out">
            <LinkBtn href="#" on:click=on_click_go_to_top>
                <i class="fa-solid fa-chevron-up"/>
            </LinkBtn>
        </div>
    }
}
