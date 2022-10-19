use gloo_utils::document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use web_sys::window;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<Input>();
}

enum Msg {
    Update,
}

struct Input {
    value: String,
}

impl Component for Input {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: "World".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                let input = document().get_element_by_id("input").unwrap();
                let input = input.dyn_into::<HtmlInputElement>().ok().unwrap();
                let input_value = input.clone().value();

                spawn_local(async move {
                    match hello(input_value).await {
                        Ok(message) => {
                            let value = message.as_string().unwrap();
                            let heading = document().get_element_by_id("heading").unwrap();
                            heading.set_inner_html(&value);
                            console::log_1(&JsValue::from(value));
                        }
                        Err(e) => {
                            let window = window().unwrap();
                            window
                                .alert_with_message(&format!("Error: {:?}", e))
                                .unwrap();
                        }
                    };
                });
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onchange = link.callback(|_| Msg::Update);
        html! {
            <div>
                <h2 class={"heading"} id={"heading"}>{"Hello "}{&self.value}</h2>
                <input {onchange} type={"text"} id={"input"} />
            </div>
        }
    }
}
