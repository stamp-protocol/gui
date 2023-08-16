use crate::{
    components::util::icon::Icon,
};
use leptos::*;
use leptos_router::*;

#[derive(Clone, Debug, Default)]
struct Action {
    id: String,
    element: View,
}

#[component]
pub fn Header(
    title: String,
    #[prop(default = "")]
    back_url: &'static str,
    #[prop(default = false)]
    transparent: bool,
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let (sig_title, _) = create_signal(title);
    let sig_action: RwSignal<Vec<Action>> = create_rw_signal(Vec::new());
    provide_context(sig_action);
    let toggle_sidebar = |e| {
        log!("sidebar: {:?}", e);
    };
    let menu_or_back = move || {
        if back_url == "" {
            view! {
                <a href="#sidebar" on:click=toggle_sidebar class="flex items-center">
                    <Icon class="text-primary text-xl relative" name="hamburger" />
                    <Show
                        when=move || !transparent
                        fallback=|| ()
                    >
                        <h1 class="font-mono font-bold ml-4 mt-1">{sig_title}</h1>
                    </Show>
                </a>
            }.into_view()
        } else {
            view! {
                <A href=back_url class="flex items-center">
                    <Icon class="text-primary" name="arrow" />
                    <Show
                        when=move || !transparent
                        fallback=|| ()
                    >
                        <h1 class="font-mono font-bold ml-4 mt-1">{sig_title}</h1>
                    </Show>
                </A>
            }.into_view()
        }
    };
    let actions = move || {
        match use_context::<RwSignal<Vec<Action>>>() {
            Some(read) => {
                view! {
                    <For
                        each=move || read.get()
                        key=|a| a.id.clone()
                        view=move |action| action.element />
                }.into_view()
            }
            _ => ().into_view()
        }
    };
    let base = "absolute top-0 w-full h-12 flex items-center justify-between px-3 z-50";
    let bg_mode = if transparent {
        "bg-white dark:bg-black text-black dark:text-white"
    } else {
        "bg-slate-800 dark:bg-black text-white"
    };
    let class = format!("{base} {bg_mode}");
    view! {
        <div class=class>
            {menu_or_back}
            <div class="nav">
               <A href="/">"Home"</A>
               <A href="/error?title=Splines+diminished&desc=Splines+could+not+be+reticulated.">"Error"</A>
               <A href="/gasp">"404"</A>
            </div>
            <div class="flex items-center">
                {actions}
            </div>
            {children.map(|c| c().into_view()).unwrap_or_else(|| ().into_view())}
        </div>
    }
}

#[component]
pub fn HeaderAction(
    id: String,
    title: String,
    icon: &'static str,
    href: &'static str,
) -> impl IntoView { 
    match use_context::<RwSignal<Vec<Action>>>() {
        Some(write) => {
            write.update(|actions| {
                let element = view! {
                    <A class="block w-8 text-center text-xl" href=href>
                        <Icon name=icon />
                    </A>
                }.into_view();
                actions.push(Action { id, element });
            });
        }
        _ => {}
    }
}

