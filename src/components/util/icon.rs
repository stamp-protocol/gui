use leptos::*;

#[component]
pub fn Icon(
    cx: Scope,
    name: &'static str,
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let entity = match name {
        "plus" => "&#xE804;",
        _ => "",
    };
    if entity == "" {
        None
    } else {
        let classname = format!("text-xl pr-2 {}", class);
        Some(leptos::html::custom(cx, leptos::html::Custom::new("icon"))
            .inner_html(entity)
            .attr("class", classname)
            .into_any())
    }
}

