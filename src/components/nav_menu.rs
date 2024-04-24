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

    let items = vec![
        NavItem {
            href: "/",
            fa_icon: "home",
            label: "Home",
        },
        NavItem {
            href: "about",
            fa_icon: "user-circle",
            label: "Sobre",
        },
        NavItem {
            href: "posts",
            fa_icon: "book",
            label: "Posts",
        },
        NavItem {
            href: "projects",
            fa_icon: "cogs",
            label: "Projetos",
        },
    ];

    let linkItems = items
        .into_iter()
        .map(|i| view! { <NavLinkItem item=i/> })
        .collect_view();

    view! {
        <nav class="tw-fixed tw-top-6 tw-left-6 tw-z-20 tw-max-w-44">
            <Show
                when=display_menu
                fallback=move || view! { <NavButton on:click=show_menu /> }
            >
                <ul node_ref=menu_ref class="tw-bg-graphite tw-text-white tw-text-sm tw-rounded-lg tw-w-full tw-overflow-hidden">
                    {linkItems.clone()}
                </ul>
            </Show>
        </nav>
    }
}

#[component]
fn NavLinkItem(item: NavItem) -> impl IntoView {
    view! {
        <li class="tw-flex hover:tw-bg-gray-700">
            <A href={item.href} class="tw-px-5 tw-py-3 tw-flex tw-flex-row tw-gap-1 tw-w-full tw-items-center">
                <div class="tw-w-5 tw-h-5">
                    <i class={format!("fa fa-{0}", item.fa_icon)}></i>
                </div>
                <span>{item.label}</span>
            </A>
        </li>
    }
}

#[component]
fn NavButton() -> impl IntoView {
    view! {
        <button class="tw-bg-graphite tw-w-12 tw-h-11 tw-bg-opacity-80 tw-text-white tw-text-3xl tw-rounded">
            <i class="fa-solid fa-bars"></i>
        </button>
    }
}

#[derive(Clone)]
struct NavItem {
    href: &'static str,
    fa_icon: &'static str,
    label: &'static str,
}
