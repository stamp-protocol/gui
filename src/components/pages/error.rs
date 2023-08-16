use crate::{
    components::{
        header::Header,
        util::{
            icon::Icon,
            page::Page,
        }
    },
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Error(
    #[prop(optional)]
    title: Option<String>,
    #[prop(optional)]
    desc: Option<String>,
) -> impl IntoView {
    let title = title
        .or_else(|| {
            use_query_map().get().get("title")
                .map(|x| x.clone())
        })
        .unwrap_or_else(|| "Unknown error".into());
    let desc = desc
        .or_else(|| {
            use_query_map().get().get("desc")
                .map(|x| x.clone())
        })
        .unwrap_or_else(|| "Unknown error".into());
    view! {
        <Header
            title=title
            transparent=false />
        <Page>
            <div class="flex justify-center py-16">
                <Icon name="sad" class="text-6xl text-red-300" />
            </div>
            <div class="prose max-w-full text-center">
                <p class="text-2xl pb-2">{desc}</p>
                <A href="/">Go to main screen</A>
            </div>
        </Page>
    }
}

