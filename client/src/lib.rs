extern crate console_error_panic_hook;

mod program_info;
use program_info::ProgramInfo;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

// This can be changed latter
static FRAG_SHADER: &'static str = include_str!("shaders/fragment.glsl");
static VERT_SHADER: &'static str = include_str!("shaders/vertex.glsl");

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    // Get canvas
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Get webgl context
    let context = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    // Use ProgramInfo to keep things tidy
    let program_info = ProgramInfo::new(&context, FRAG_SHADER, VERT_SHADER)?;
    context.use_program(Some(&program_info.program));

    // The points of the triangle
    let vertices: [f32; 12] = [
        -1.0, -1.0,
         1.0, -1.0,
        -1.0,  1.0,
        -1.0,  1.0,
         1.0, -1.0,
         1.0,  1.0,
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
    // TODO: Change 2 to 3 after move to 3d
    context.vertex_attrib_pointer_with_i32(a_position_location, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(a_position_location);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        // TODO: Change 2 to 3 after move to 3d
        (vertices.len() / 2) as i32,
    );

    Ok(())
}
