use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*};
use crate::routes::AppRouter;

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Home {
    _link: ComponentLink<Self>,
}

pub enum Msg {
    FormSubmit,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Home { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::FormSubmit => {
                console_log!("Form Submit");
            },
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // <div class="d-flex justify-content-center">
                <div class="text-center">
                    <h3>{"Content goes here"}</h3>
                    <form onsubmit=self._link.callback(|e: FormData|{
                        e.prevent_default();
                        Msg::FormSubmit
                    }) autocomplete="off">
                        <input type="text" name="entrada" value="" />
                    </form>
                    <RouterAnchor<AppRouter> classes="btn btn-warning btn-sm text-light" route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>>
                </div>
            </div>
        }
    }
}
