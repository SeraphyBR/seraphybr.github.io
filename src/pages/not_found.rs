use leptos::prelude::*;

use crate::pages::base::BasePage;

#[component]
pub fn NotFoundPage() -> impl IntoView {
  view! {
      <BasePage title="404">
          <div class="tw-flex tw-justify-center tw-items-center tw-m-9">
              <div class="tw-text-center">
                  <i class="fas fa-compass tw-text-5xl tw-mb-4"></i>
                  <h1 class="tw-text-3xl tw-font-bold tw-mb-2">404</h1>
                  <p class="tw-text-xl">A página não existe, ou não é mais acessivel!<br/>Use o botão de navegação para voltar.</p>
              </div>
          </div>
      </BasePage>
  }
}
