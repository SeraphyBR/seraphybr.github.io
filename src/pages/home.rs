use leptos::*;

use crate::pages::base::BasePage;

/// Default Home Page
#[component]
pub fn HomePage() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <BasePage>
            <div class="tw-flex tw-justify-center tw-items-center tw-m-9">
                <div class="tw-text-center">
                    <i class="fas fa-tools tw-text-5xl tw-mb-4"></i>
                    <h1 class="tw-text-3xl tw-font-bold tw-mb-2">Em Construção</h1>
                    <p class="tw-text-lg">Estamos trabalhando para melhorar sua experiência. Volte em breve!</p>
                    <input type="number" on:input=on_input/>
                    <p>"You entered " <strong>{value}</strong></p>
                </div>
            </div>
        </BasePage>
    }
}
