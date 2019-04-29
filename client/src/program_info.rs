use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

// Used to keep webgl shaders/programs tidy
pub struct ProgramInfo {
    frag_shader: WebGlShader,
    vert_shader: WebGlShader,
    program: WebGlProgram,
}

impl ProgramInfo {

    pub fn new(
        context: &WebGl2RenderingContext, 
        frag_source: &str, 
        vert_soruce: &str,
    ) -> Result<ProgramInfo, String> {

        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            frag_source,
        )?;

        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            vert_soruce,
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;

        Ok(ProgramInfo {
            frag_shader,
            vert_shader,
            program,
        })
    }
}

// The folowing two functions were adapted from:
// https://rustwasm.github.io/wasm-bindgen/examples/webgl.html

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {

    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let compile_status = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);

    if compile_status {
        Ok(shader)
    } else {

        let error = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));

        Err(error)
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {

    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    let link_status = context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);

    if link_status {
        Ok(program)
    } else {

        let error = context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"));

        Err(error)
    }
}
