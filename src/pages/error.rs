use leptos::*;

use crate::pages::base::BasePage;

#[component]
pub fn ErrorBoundaryPage(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=error>
            {children()}
        </ErrorBoundary>
    }
}

fn error(errors: RwSignal<Errors>) -> impl IntoView {
    view! {
        <BasePage>
            <div class="tw-flex tw-flex-col tw-gap-4">
                <div class="tw-flex tw-flex-col tw-gap-1 tw-justify-center tw-items-center tw-text-red-900 tw-text-3xl tw-font-bold">
                    <i class="fa-solid fa-exclamation-circle"></i>
                    <h1>
                        "Ops! Algo deu errado!"
                    </h1>
                </div>

                <p>
                    {"Um erro inesperado aconteceu. Por favor, tente novamente recarregando a página, e reporte o erro."}
                </p>

                <p class="tw-text-lg tw-font-medium">"Erros:"</p>

                // Renderiza uma lista de erros como strings - útil para propósitos de desenvolvimento
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            </div>
        </BasePage>
    }
}
