use leptos::*;

use crate::{components::button::LinkBtn, pages::base::BasePage};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <BasePage title="Home" class="!tw-max-w-lg">
            <div class="tw-vflex tw-justify-center tw-items-center tw-gap-2">
                <img src="/img/logo.png" class="tw-mx-auto tw-rounded-[50%] tw-w-36 tw-h-36"/>
                <h3 class="tw-text-xl tw-font-light">"SeraphyBR's Blog"</h3>
                <hr />
                <h3 class="tw-text-base tw-font-light">"Desenvolvedor - Página em construção"</h3>
                <hr />
                <div class="tw-hflex tw-gap-5">
                    <LinkBtn href="mailto:luisjuniorbr@gmail.com" class="!tw-text-3xl">
                        <i class="fa fa-envelope-square"/>
                    </LinkBtn>

                    <LinkBtn href="https://github.com/seraphybr" class="!tw-text-3xl">
                        <i class="fa-brands fa-github"/>
                    </LinkBtn>

                    <LinkBtn href="https://t.me/seraphybr" class="!tw-text-3xl">
                        <i class="fa-brands fa-telegram"/>
                    </LinkBtn>
                </div>
                <hr />
                <div class="tw-hflex tw-gap-5">
                    <LinkBtn href="about" class="!tw-text-base !tw-font-normal !tw-min-w-24">
                        Sobre
                    </LinkBtn>

                    <LinkBtn href="posts" class="!tw-text-base !tw-font-normal !tw-min-w-24">
                        Posts
                    </LinkBtn>

                    <LinkBtn href="projects" class="!tw-text-base !tw-font-normal !tw-min-w-24">
                        Projetos
                    </LinkBtn>
                </div>
            </div>
        </BasePage>
    }
}
