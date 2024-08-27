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
    let options = ScrollToOptions::default();
    options.set_behavior(ScrollBehavior::Smooth);
    options.set_top(0.0);
    window().scroll_to_with_scroll_to_options(&options);
  };

  view! {
      <div style:opacity=opacity_btn class="tw-fixed tw-bottom-6 tw-right-6 tw-transition-opacity tw-duration-700 tw-ease-in-out">
          <button on:click=on_click_go_to_top class="tw-btn-primary tw-bg-opacity-80 tw-backdrop-blur-md">
              <i class="fa-solid fa-chevron-up"/>
          </button>
      </div>
  }
}
