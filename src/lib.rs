use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod contexts;
mod markdown_posts;
mod models;
mod pages;

use crate::components::nav_menu::NavMenu;
use crate::contexts::posts::PostsProvider;
use crate::pages::about::AboutPage;
use crate::pages::error::ErrorBoundaryPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::posts::{PostListPage, PostPage};
use crate::pages::projects::{ProjectPage, ProjectsListPage};

use crate::components::dark_mode_toggle::DarkModeToggleBtn;

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
        <Html lang="pt-br" dir="ltr" />

        // sets the document title
        <Title formatter=title_formatter />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <ErrorBoundaryPage>
            <PostsProvider>
                <Router>
                    <NavMenu />
                    <DarkModeToggleBtn />
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="posts" view=PostListPage/>
                        <Route path="posts/:path" view=PostPage/>
                        <Route path="projects" view=ProjectsListPage/>
                        <Route path="projects/:path" view=ProjectPage/>
                        <Route path="about" view=AboutPage/>
                        <Route path="404" view=NotFoundPage/>
                        <Route path="/*" view=NotFoundPage/>
                    </Routes>
                </Router>
            </PostsProvider>
        </ErrorBoundaryPage>
    }
}
