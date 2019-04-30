
mod program_info;
use program_info::ProgramInfo;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::JsCast;

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
}
