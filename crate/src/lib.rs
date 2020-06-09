#![recursion_limit = "512"]

#[macro_use]
extern crate cfg_if;

mod routes;
mod app;
mod pages;
mod todo;
mod todo_model;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
extern crate yew_router;

use wasm_bindgen::prelude::*;
// use js_sys::*;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    console_log!("SISN Application");
}

pub fn get_element(selector: &str) -> web_sys::Element {
    let document = yew::utils::document();
    document.query_selector(selector).unwrap().unwrap()
}

/// A função `mount_at` deve receber o seletor de onde vai ser montado o componente
///
/// # Mount At
///
/// ```
/// <script>
///     mount_at("#app");
/// </script>
/// ```
#[wasm_bindgen]
pub fn mount_at(selector: &str) {
    let _document = yew::utils::document();
    // let mount_point = document.query_selector(selector).unwrap().unwrap();
    let mount_point = get_element(selector);
    // let window = yew::utils::document();
    log("Second Step to Debug");
    yew::App::<app::App>::new().mount(mount_point);
}

#[wasm_bindgen] /*-> Result<(), JsValue>*/
pub fn mount_here(selector: &str){
    log(selector);
}
