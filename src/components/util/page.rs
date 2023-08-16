use leptos::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="page absolute left-0 right-0 top-12 bottom-0 overflow-auto p-8 bg-white dark:bg-black z-40">
            {children()}
        </div>
    }
}

