#![recursion_limit = "512"]

mod app;

#[macro_use]
mod state;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<state::StateContainer>();
    Ok(())
}
