use leptos::*;

use crate::pages::base::BasePage;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <BasePage>
            <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>
        </BasePage>
    }
}
