use leptos::*;
use leptos_router::A;

#[component]
pub fn NavMenu() -> impl IntoView {
    view! {
        <nav class="tw-fixed tw-top-6 tw-left-6 tw-z-20 tw-max-w-44">
            <ul class="tw-bg-graphite tw-text-white tw-text-sm tw-rounded-lg">
                <li>
                    <A href="/" class="tw-px-5 tw-py-2">
                        <i class="fa fa-home"></i> Home
                    </A>
                </li>
                <li>
                    <A href="#">
                        <i class="fa fa-user-circle"></i> Sobre
                    </A>
                </li>
                <li>
                    <A href="#">
                        <i class="fa fa-book"></i> Posts
                    </A>
                </li>
                <li>
                    <A href="projects">
                        <i class="fa fa-cogs"></i> Projetos
                    </A>
                </li>
            </ul>
        </nav>
    }
}
