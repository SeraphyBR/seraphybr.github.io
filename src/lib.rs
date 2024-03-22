use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

use crate::components::nav_menu::NavMenu;
use crate::pages::error::ErrorBoundaryPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let title_formatter = |text: String| {
        let base_title: String = "SeraphyBR`s Blog".into();

        if text.is_empty() {
            base_title
        } else {
            format!("{text} - {base_title}")
        }
    };

    view! {
        <Html lang="pt-br" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title formatter=title_formatter />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Body
            class="tw-bg-no-repeat tw-bg-cover tw-h-full tw-bg-center tw-bg-fixed tw-bg-gray-800"
            attr:style="background-image: url('img/background-1.jpg')"
        />

        <ErrorBoundaryPage>
            <Router>
                <NavMenu />
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/*" view=NotFoundPage/>
                </Routes>
            </Router>
        </ErrorBoundaryPage>
    }
}
