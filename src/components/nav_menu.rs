use leptos::*;
use leptos_router::A;
use leptos_use::on_click_outside;

#[component]
pub fn NavMenu() -> impl IntoView {
    let (display_menu, set_display_menu) = create_signal(false);

    let show_menu = move |_| set_display_menu(true);
    let close_menu = move |_| set_display_menu(false);

    let menu_ref = create_node_ref::<html::Ul>();

    let _ = on_click_outside(menu_ref, close_menu);

    view! {
        <nav class="tw-fixed tw-top-6 tw-left-6 tw-z-20 tw-max-w-44">
            <Show
                when=display_menu
                fallback=move || view! { <NavButton on:click=show_menu /> }
            >
                <ul node_ref=menu_ref class="tw-bg-graphite tw-text-white tw-text-sm tw-rounded-lg">
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
            </Show>
        </nav>
    }
}

#[component]
fn NavButton() -> impl IntoView {
    view! {
        <button class="tw-bg-graphite tw-w-12 tw-h-11 tw-opacity-60 tw-text-white tw-text-3xl tw-rounded">
            <i class="fa-solid fa-bars"></i>
        </button>

    }
}
