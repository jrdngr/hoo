use yew::{html, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;

struct App {
    clicked: bool,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App { clicked: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };

        html! {
            <button onclick=|_| Msg::Click>{ button_text }</button>
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    yew::start_app::<App>();
}
