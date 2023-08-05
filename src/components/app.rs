use crate::{
    components::{
        form::button::{Button, ButtonType},
    },
    models::core::invoke,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (feedback, set_feedback) = create_signal(cx, String::new());
    create_effect(cx, move |_| {
        let val = count.get();
        let change = if val == 3 {
            Some("Don't click it twice and three times, Daniel!!")
        } else if val == 10 {
            Some("Stop!!")
        } else if val == 20 {
            Some("Stop clicking it!!!!")
        } else if val == 30 {
            Some("You know, what is wrong with you, Daniel??")
        } else if val == 40 {
            Some("Don't click it 40 times, Daniel!!")
        } else {
            None
        };
        if let Some(msg) = change {
            spawn_local(async move {
                let args = to_value(&GreetArgs { name: msg }).unwrap();
                let feed = invoke("greet", args).await.as_string().unwrap();
                set_feedback.set(feed);
            });
        }
    });

    #[derive(Serialize, Deserialize)]
    struct GreetArgs<'a> {
        name: &'a str,
    }

    view! {
        cx,
        <div class="p-8 prose text-black dark:text-white">
            <Button
                on:click=move |_| { set_count.update(|n| *n += 1); }
                icon="plus"
            >
                "Just click it one time, Daniel!! Just once!!!"
            </Button>
            <br/>
            <span>"Clicked " {count} " times"</span>
            <br/>
            {feedback}
        </div>
    }
}

