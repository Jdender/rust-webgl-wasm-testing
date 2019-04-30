extern crate console_error_panic_hook;

mod webgl;
use webgl::WebGl;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    // Get canvas
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // TODO: Slowly move common things into WebGl
    let gl = WebGl::new(canvas)?;

    // Unpack these to make it easier to copy/paste
    let context = gl.context;
    let program_info = gl.program_info;

    // The points of the triangle
    let vertices: Vec<f32> = vec![
        0.0, 0.0, 0.0,
        0.0, 0.5, 0.0,
        0.7, 0.0, 0.0,
    ];

    // Creat a wasm buffer
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();

    // Turn the wasm buffer into a Float32Array and put the vertices into it
    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);

    // Get location of a_position
    let a_position_location = context.get_attrib_location(&program_info.program, "a_position") as u32;

    // Create a webgl bugger and put the wasm buffer into it
    let buffer = context.create_buffer().ok_or("failed to create webgl buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    // Set a_position to the values of the webgl buffer
    context.vertex_attrib_pointer_with_i32(a_position_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(a_position_location);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    Ok(())
}
