use yew::prelude::*;
use yew_router::{prelude::*};
use crate::routes::AppRouter;

pub struct NavBar {
    _link: ComponentLink<Self>,
}

impl Component for NavBar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavBar { _link }
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
                <nav class="navbar navbar-expand-lg navbar-light bg-light">
                    <a class="navbar-brand" href="#">{"Dashboard"}</a>
                    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent"
                        aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav mr-auto">
                            <li class="nav-item active">
                                // <a class="nav-link" href="/">{"Home"} <span class="sr-only">{"(current)"}</span></a>
                                <RouterAnchor<AppRouter> classes="nav-link" route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>>
                            </li>
                            <li class="nav-item">
                                // <a class="nav-link" href="#">{"Link"}</a>
                                <RouterAnchor<AppRouter> classes="nav-link" route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>>
                            </li>
                            // <li class="nav-item dropdown">
                            //     <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button"
                            //         data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                            //         {"Dropdown"}
                            //     </a>
                            //     <div class="dropdown-menu" aria-labelledby="navbarDropdown">
                            //         <a class="dropdown-item" href="#">{"Action"}</a>
                            //         <a class="dropdown-item" href="#">{"Another action"}</a>
                            //         <div class="dropdown-divider"></div>
                            //         <a class="dropdown-item" href="#">{"Something else here"}</a>
                            //     </div>
                            // </li>
                            <li class="nav-item">
                                <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">{"Disabled"}</a>
                            </li>
                        </ul>
                    </div>
                </nav>
            </div>
        }
    }
}
