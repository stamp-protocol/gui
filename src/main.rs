pub mod components;
pub mod models;

use leptos::*;
use crate::components::app::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

