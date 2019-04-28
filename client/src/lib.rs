extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
          
    let context = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    Ok(())
}
