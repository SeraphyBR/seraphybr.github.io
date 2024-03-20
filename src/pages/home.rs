use crate::components::background::Background;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Background>
                <div class="tw-bg-gray-200 tw-flex tw-justify-center tw-items-center tw-h-screen">
                    <div class="tw-text-center">
                        <i class="fas fa-tools tw-text-5xl tw-mb-4"></i>
                        <h1 class="tw-text-3xl tw-font-bold tw-mb-2">Em Construção</h1>
                        <p class="tw-text-lg">Estamos trabalhando para melhorar sua experiência. Volte em breve!</p>
                    </div>
                </div>
            </Background>
        </ErrorBoundary>
    }
}
