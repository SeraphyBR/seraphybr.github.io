use leptos::*;

#[component]
pub fn Divider() -> impl IntoView {
  view! {
      <hr class="!tw-border-t-0 tw-w-full tw-h-px tw-bg-gradient-to-r tw-from-transparent tw-via-black dark:tw-via-white tw-to-transparent tw-opacity-15 tw-my-2"/>
  }
}
