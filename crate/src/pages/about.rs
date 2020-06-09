use yew::prelude::*;
use yew_router::{prelude::*};
use crate::routes::AppRouter;

pub struct About {
    _link: ComponentLink<Self>,
}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        About { _link }
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
                {"About Page"}
                <RouterAnchor<AppRouter> classes="btn btn-info btn-sm" route=AppRouter::RootPath>{"Back"}</RouterAnchor<AppRouter>>
            </div>
        }
    }
}
