#![recursion_limit = "1024"]

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
extern crate yew_router;

use wasm_bindgen::prelude::*;

mod todo;

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
pub fn main(){
    set_panic_hook();
    yew::start_app::<todo::TodoModel>();
}

// #[wasm_bindgen]
// pub fn main(){

//     // YEW BEGIN
//     yew::initialize();
//     let _yew_document = yew::utils::document();
//     // let yew_mount_point = yew_document.query_selector("#todo-app").unwrap().unwrap();
//     // let yew_mount_point: web_sys::Element = yew_document.query_selector("#todo-app");
    
//     // YEW END


//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let window = web_sys::window().unwrap();
//     let document = window.document().unwrap();
//     let body = document.body().unwrap();

//     // Manufacture the element we're gonna append
//     let val = document.create_element("div").unwrap();
//     val.set_inner_html("WASM Loaded!");
//     body.append_child(&val)?;

//     // App::<todo::TodoModel>::new().mount(val);
//     // App::<todo::TodoModel>::new().mount_to_body();
//     yew::start_app::<todo::TodoModel>();

//     // Ok(())
// }