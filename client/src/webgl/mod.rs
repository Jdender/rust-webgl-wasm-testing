
mod program_info;
use program_info::ProgramInfo;

mod matrix;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;

// These are static for now, I'll change it to pass to new() as args if needed later
static FRAG_SHADER: &'static str = include_str!("shaders/fragment.glsl");
static VERT_SHADER: &'static str = include_str!("shaders/vertex.glsl");

// Used to provide a abstraction over webgl
pub struct WebGl {
    pub context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    pub program_info: ProgramInfo,
}

impl WebGl {

    pub fn new(canvas: HtmlCanvasElement) -> Result<WebGl, String> {

        // Get webgl context
        // The methods are full of Result & Option so lots of unwraping
        let context = canvas.get_context("webgl2")
            .map_err(|_| String::from("Unable to create webgl2 context"))?.unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();

        // Program/shaders deserve their own struct
        let program_info = ProgramInfo::new(&context, FRAG_SHADER, VERT_SHADER)?;

        // Set the program as the active one
        context.use_program(Some(&program_info.program));

        Ok(WebGl {
            context,
            canvas,
            program_info,
        })
    }

    pub fn resize_canvas(&self) {

        // Get the size of the canvas on the page
        let display_width = self.canvas.client_width() as u32;
        let display_height = self.canvas.client_height() as u32;

        // Compare agenst the size of it's buffer
        let is_same_size = self.canvas.width() == display_width && self.canvas.height() == display_height;

        // Grow or shrink the buffer to fit the canvas on the page
        if !is_same_size {

            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
        }
    }
}

use js_sys::WebAssembly;

pub fn vec_to_float_array(vec: &Vec<f32>) -> Result<js_sys::Float32Array, JsValue> {

    // Create a wasm buffer
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();

    // Make a js typed array the size of the buffer
    let float_array = js_sys::Float32Array::new(&memory_buffer);

    // Get a pointer
    let vertices_location = vec.as_ptr() as u32 / 4;

    // Fill the buffer with the vector
    Ok(float_array.subarray(vertices_location, vertices_location + vec.len() as u32))
}
