use leptos::*;

use crate::pages::base::BasePage;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <BasePage title="404">
            <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>
        </BasePage>
    }
}
