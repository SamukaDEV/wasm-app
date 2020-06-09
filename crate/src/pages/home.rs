use yew::prelude::*;
use yew_router::{prelude::*};
use crate::routes::AppRouter;

pub struct Home {
    _link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Home { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {"Home Page"}
                <RouterAnchor<AppRouter> classes="btn btn-warning btn-sm" route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>>
            </div>
        }
    }
}
