// #![recursion_limit = "256"]
// #![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

// #[macro_use]
// extern crate json;
// extern crate stdweb;
extern crate serde_json;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = localStorage, js_name = setItem)]
    fn localStorage_setItem(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage, js_name = getItem)]
    fn localStorage_getItem(key: &str) -> String;

    #[wasm_bindgen(js_namespace = JSON, js_name = stringify)]
    fn json_stringify(obj: &str) -> String;
}


pub struct TodoModel {
    input: String,
    todos: Vec<String>,
    // todos: json::JsonValue,
}

pub enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Debug,
    SaveAll,
    Nothing,
}

impl Component for TodoModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TodoModel { // Self {
            input: "".to_string(),
            todos: {
                let json = localStorage_getItem("TODOS");
                let res = json::parse(&json).unwrap();
                let mut n = vec![];
                for v in res.members(){
                    n.push(v.to_string());
                };
                n
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let s = self.input.clone();
                self.todos.push(s);
                self.input = "".to_string();
                // self.link.send_message(Msg::SaveAll);
            }
            Msg::Update(s) => {
                self.input = s;
            }
            Msg::Remove(i) => {
                self.todos.remove(i);
                // self.link.send_message(Msg::SaveAll);
            }
            Msg::RemoveAll => {
                self.todos = vec![];
                // self.link.send_message(Msg::SaveAll);
            }
            Msg::SaveAll => {
                localStorage_setItem("TODOS", &json::stringify(self.todos.to_vec()));
                log("Saved");
            }
            Msg::Debug => {}
            Msg::Nothing => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view_todo = |(index, todo): (usize, &String)| {
            html! {
                <li class="ui-state-default">
                    <div class="checkbox d-flex justify-content-between">
                        <label>
                            <input type="checkbox"/>
                            {format!(" {}", &todo)}
                        </label>
                        <button class="btn btn-danger btn-sm"
                        // onclick=self.link.callback(move |_|{
                        //     Msg::Remove(index)
                        // }) 
                        >{"X"}</button>
                    </div>
                </li>
            }
        };
        html! {
            <div>
                <div class="row">
                    <div class="col-md-6">
                        <div class="todolist not-done">
                            <h1>{"TODO's"}</h1>
                            <input type="text" class="form-control add-todo" placeholder="Add todo",
                            value=self.input,
                            // oninput=self.link.callback(|e: InputData| Msg::Update(e.value)),
                            // onkeypress=self.link.callback(|e: KeyboardEvent|{
                            //     if e.key() == "Enter" {Msg::Add} else {Msg::Nothing}
                            // }),
                            // onchange=self.link.callback(|e: ChangeData| Msg::ChangeValue({
                            //     // log(e);
                            //     "aaa".to_string()
                            // })),
                            />
                            // <p>{&self.input}</p>
                            // <hr />
                            <br />
                            <ul id="sortable" class="list-unstyled">
                                {
                                    for self.todos.iter().enumerate().map(view_todo)
                                }
                                // <li class="ui-state-default">
                                //     <div class="checkbox">
                                //         <label>
                                //             <input type="checkbox" value="" />{"Take out the trash"}</label>
                                //     </div>
                                // </li>
                                // <li class="ui-state-default">
                                //     <div class="checkbox">
                                //         <label>
                                //             <input type="checkbox" value="" />{"Buy bread"}</label>
                                //     </div>
                                // </li>
                                // <li class="ui-state-default">
                                //     <div class="checkbox">
                                //         <label>
                                //             <input type="checkbox" value="" />{"Teach penguins to fly"}</label>
                                //     </div>
                                // </li>
                            </ul>
                            <div class="todo-footer d-flex justify-content-between">
                                <span class="mt-2">{"0 Items Left"}</span>
                                <button class="btn btn-success" 
                                // onclick=self.link.callback(|_|Msg::RemoveAll)
                                >{"Delete All"}</button>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-6">
                        <div class="todolist">
                            <h1>{"Already Done"}</h1>
                            <ul id="done-items" class="list-unstyled">
                                <li>{"Some item"}<button class="remove-item btn btn-default btn-xs pull-right"><span class="glyphicon glyphicon-remove"></span></button></li>
                            </ul>
                        </div>
                        <button class="btn btn-info btn-sm" 
                        // onclick=self.link.callback(|_|Msg::Debug), 
                        >{"Debug"}</button>
                    </div>
                </div>
            </div>
        }
    }
}