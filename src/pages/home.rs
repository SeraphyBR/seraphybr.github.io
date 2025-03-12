use leptos::prelude::*;
use leptos_router::components::A;

use crate::{components::divider::Divider, pages::base::BasePage};

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
      <BasePage title="Home" class="!tw-max-w-lg">
          <div class="tw-vflex tw-justify-center tw-items-center tw-gap-2">
              <img src="/img/logo.png" class="tw-mx-auto tw-rounded-[50%] tw-w-36 tw-h-36 tw-animate-rotateIn"/>
              <h3 class="tw-text-xl tw-font-light">"SeraphyBR's Blog"</h3>
              <Divider />
              <h3 class="tw-text-base tw-font-light">"Desenvolvedor - Página em construção"</h3>
              <Divider />
              <div class="tw-hflex tw-gap-5">
                  <A href="mailto:luisjuniorbr@gmail.com" attr:class="tw-btn-primary !tw-text-3xl">
                      <i class="fa fa-envelope-square"/>
                  </A>

                  <A href="https://github.com/seraphybr" attr:class="tw-btn-primary !tw-text-3xl">
                      <i class="fa-brands fa-github"/>
                  </A>

                  <A href="https://t.me/seraphybr" attr:class="tw-btn-primary !tw-text-3xl">
                      <i class="fa-brands fa-telegram"/>
                  </A>
              </div>
              <Divider />
              <div class="tw-hflex tw-gap-5">
                  <A href="about" attr:class="tw-btn-primary !tw-text-base !tw-font-normal !tw-min-w-24">
                      Sobre
                  </A>

                  <A href="posts" attr:class="tw-btn-primary !tw-text-base !tw-font-normal !tw-min-w-24">
                      Posts
                  </A>

                  <A href="projects" attr:class="tw-btn-primary !tw-text-base !tw-font-normal !tw-min-w-24">
                      Projetos
                  </A>
              </div>
          </div>
      </BasePage>
  }
}
