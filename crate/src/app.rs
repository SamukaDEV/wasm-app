#[warn(unused_imports)]

use crate::pages::{About, Home};
use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive};
use crate::routes::AppRouter;

pub struct App {
    _link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App { _link }
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
                // <nav class="navbar navbar-expand-lg navbar-light bg-light">
                //     <RouterAnchor<AppRouter> classes="navbar-brand" route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>>
                //     <div class="collapse navbar-collapse">
                //         <ul class="navbar-nav mr-auto">
                //             <li class="nav-item">
                //                 <RouterAnchor<AppRouter> classes="nav-link" route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>>
                //             </li>
                //             <li class="nav-item">
                //                 <RouterAnchor<AppRouter> classes="nav-link" route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>>
                //             </li>
                //         </ul>
                //     </div>
                // </nav>
                <div class="container">
                    <Router<AppRouter, ()>
                        render = Router::render(|switch: AppRouter | {
                            match switch {
                                AppRouter::RootPath => html!{
                                    <Home />
                                },
                                AppRouter::AboutPath => html!{
                                    <About />
                                },
                                AppRouter::PageNotFound(Permissive(None)) => html!{"Page Not Found!"},
                                AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page {} Not Found", missed_route)}
                            }
                        })
                    />
                    <hr/>
                    <RouterAnchor<AppRouter> route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>>
                    <RouterAnchor<AppRouter> route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>>
                </div>
            </div>
        }
    }
}
