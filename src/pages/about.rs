use leptos::*;

use crate::{components::content::BaseContent, pages::base::BasePage};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <BasePage title="Sobre mim">
            <BaseContent
                title="Sobre mim"
                bg_color=Some("#4ce16599".to_string())
                bg_img=Some("url('/img/aboutme.jpg')".to_string())
                back_href="/"
            >
            Teste
            </BaseContent>
        </BasePage>
    }
}
