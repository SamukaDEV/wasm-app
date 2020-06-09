use yew::prelude::*;

pub struct App{
    link: ComponentLink<Self>,
}

pub enum AppMsg{
    Debug
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {link}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::Debug => {
                print!("Nothing ");
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div> 
                <div class="bg-light border rounded p-3 m-3">
                    {"Content goes here"}
                    <hr/>
                    <div class="d-flex justify-content-end">
                        <button class="btn btn-info btn-sm" 
                        onclick=self.link.callback(|_|AppMsg::Debug)
                        >{"Debug"}</button>
                    </div>
                </div>
            </div>
        }
    }
}
