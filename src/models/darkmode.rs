//! Handles setting/getting/saving darkmode!

use crate::{
    models::localstorage,
};
use leptos::*;

#[derive(Clone, Debug, Default)]
pub struct Darkmode(bool);

impl Darkmode {
    fn toggle(&mut self) { self.0 = !self.0 }
    pub fn enabled(&self) -> bool { self.0 }
}

/// Initialize the model, which includes setting it into a global state und ztill creating
/// ze proper effect.
pub fn init() -> RwSignal<Darkmode> {
    let darkmode_str = localstorage::get("app:darkmode")
        .unwrap_or_else(|| "false".into());
    // parse the JSON
    let darkmode = match darkmode_str.as_str() {
        "true" => true,
        _ => false,
    };
    let sig = create_rw_signal(Darkmode(darkmode));
    create_effect(move |_| {
        let darkmode = sig.get();
        let _ = localstorage::set("app:darkmode", if darkmode.enabled() { "true" } else { "false" });
    });
    sig
}

/// Toggle darkmode
pub fn toggle() {
    use_context::<RwSignal<Darkmode>>()
        .map(|sig| {
            sig.update(|darkmode| darkmode.toggle());
        });
}

