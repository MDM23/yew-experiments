#![recursion_limit = "512"]

mod hooks;

use hooks::{use_drag, use_drag_with_options, DragHookOptions};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_functional::{FunctionComponent, FunctionProvider};

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
        let (coords_a, node_a) = use_drag_with_options(DragHookOptions {
            event_callback: Callback::noop(),
            reset_on_drop: true,
        });

        let (coords_b, node_b) = use_drag();

        html! {
            <main>
                <div
                    ref=(*node_a).borrow().clone()
                    style={format!("left: {}px; top: {}px;", coords_a.0, coords_a.1)}
                    class="circle absolute"
                    draggable="true"
                />
                <div
                    ref=(*node_b).borrow().clone()
                    style={format!("left: {}px; top: {}px;", coords_b.0, coords_b.1)}
                    class="circle absolute"
                    draggable="true"
                />
            </main>
        }
    }
}
