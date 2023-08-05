use crate::{
    components::util::icon::Icon,
};
use leptos::*;

pub enum ButtonType {
    Filled,
    Outlined,
}

#[component]
pub fn Button(
    cx: Scope,
    children: Children,
    #[prop(default = ButtonType::Filled)]
    ty: ButtonType,
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = "")]
    icon: &'static str,
    #[prop(default = "")]
    icon_class: &'static str,
    #[prop(default = false)]
    submit: bool,
    #[prop(default = false)]
    disabled: bool,
    #[prop(optional)]
    tabindex: u16,
) -> impl IntoView {
    let style = match ty {
        ButtonType::Filled => "text-white dark:text-black bg-primary dark:bg-primary/90 drop-shadow hover:drop-shadow-md focus:drop-shadow-md hover:bg-primary-600 dark:hover:bg-primary focus:bg-primary-600 dark:focus:bg-primary disabled:cursor-not-allowed disabled:text-slate-500 disabled:bg-slate-300 disabled:drop-shadow-none dark:disabled:text-slate-800 dark:disabled:bg-slate-500",
        ButtonType::Outlined => "text-primary dark:text-primary-400 border border-primary hover:bg-primary-50 dark:hover:bg-primary-900 focus:bg-primary-50 dark:focus:bg-primary-900 disabled:cursor-not-allowed disabled:text-slate-400 disabled:border-slate-300 disabled:bg-transparent disabled:drop-shadow-none dark:disabled:text-slate-500 dark:disabled:border-slate-500 disabled:dark:bg-transparent",
    };
    let btn_type = if submit { "submit" } else { "button" };
    let classname = format!("cursor-pointer flex items-center px-4 py-2 rounded font-semibold transition-all outline-none  {} {}", style, class);
    let icon_view = move || {
        if icon != "" {
            Some(view!{ cx, <Icon name=&icon class=&icon_class />})
        } else {
            None
        }
    };
    view! { cx,
        <button
            type={btn_type}
            class=classname
            tabindex=tabindex
            disabled=disabled
        >
            {icon_view}
            <div class="relative">{children(cx)}</div>
        </button>
    }
}

