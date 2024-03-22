use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

use crate::components::nav_menu::NavMenu;
// Top-Level pages
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;

use crate::pages::error::ErrorBoundaryPage;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="pt-br" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Em construção"/>

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
