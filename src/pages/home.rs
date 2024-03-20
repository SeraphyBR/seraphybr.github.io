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
                <div class="container">
                    <p class="tw-text-xl tw-bg-slate-100">"Teste tailwind"</p>
                    <i class="fa-solid fa-blog"></i>
                </div>
            </Background>
        </ErrorBoundary>
    }
}
