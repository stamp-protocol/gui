use leptos::*;

#[component]
pub fn Icon(
    name: &'static str,
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let entity = match name {
        "arrow" => "&#xE800;",
        "gear" => "&#xE801;",
        "hamburger" => "&#xF0C9;",
        "id" => "&#xF2C3",
        "photo" => "&#xE805;",
        "plus" => "&#xE804;",
        "rolodex" => "&#xF2BC;",
        "sad" => "&#xE806;",
        "sync" => "&#xE803;",
        "tripledot" => "&#xE802;",
        "user" => "&#xF233;",
        "user_add" => "&#xF234;",
        _ => "",
    };
    if entity == "" {
        None
    } else {
        Some(leptos::html::custom(leptos::html::Custom::new("icon"))
            .inner_html(entity)
            .attr("class", class.to_string())
            .into_any())
    }
}

