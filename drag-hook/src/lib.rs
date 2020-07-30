#![recursion_limit = "512"]

mod hooks;

use hooks::{use_drag, Coordinates};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_functional::{use_ref, FunctionComponent, FunctionProvider};

#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<FunctionComponent<App>>();
    Ok(())
}

pub struct App {}

impl FunctionProvider for App {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let node_ref = use_ref(|| NodeRef::default());

        let coords = use_drag(
            node_ref.borrow().clone(),
            Callback::noop(),
            Coordinates(50, 50),
        );

        html! {
            <main>
                <div
                    ref=(*node_ref).borrow().clone()
                    style={format!("left: {}px; top: {}px;", coords.0, coords.1)}
                    class="circle absolute"
                />
            </main>
        }
    }
}
