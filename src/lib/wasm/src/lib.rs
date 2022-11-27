extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use web_sys::{WebGl2RenderingContext, WebGlBuffer};
extern crate nalgebra_glm as glm;

mod webgl;
use webgl::{compile_shader, get_context_by_id, link_shader_program, ShaderInfo, draw};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn render(vertex_shader: &str, fragment_shader: &str) -> Result<(), JsValue> {
    let (context, canvas) = get_context_by_id("canvas").unwrap();

    let vertex_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        vertex_shader,
    )?;

    let fragment_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragment_shader,
    )?;

    let program = link_shader_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let vertex_position = context.get_attrib_location(&program, "aVertexPosition") as u32;

    let model_view_matrix = context
        .get_uniform_location(&program, "uModelViewMatrix")
        .unwrap();
    let projection_matrix = context
        .get_uniform_location(&program, "uProjectionMatrix")
        .unwrap();

    let vertex_color = context.get_attrib_location(&program, "aVertexColor") as u32;
    context.enable_vertex_attrib_array(vertex_color);

    let buffers = init_buffers(&context);

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;

    context.bind_vertex_array(Some(&vao));
    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(vertex_position);
    context.bind_vertex_array(Some(&vao));

    let info = ShaderInfo {
        program: &program,
        canvas_height: canvas.height,
        canvas_width: canvas.width,
        vertex_position,
        vertex_color,
        program_projection_matrix: &projection_matrix,
        program_model_view_matrix: &model_view_matrix,
    };

    draw_colored_square(&context, &info, &buffers);

    Ok(())
}

fn init_buffers(context: &WebGl2RenderingContext) -> (WebGlBuffer, WebGlBuffer) {
    let positions = [-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

    let position_buffer = context
        .create_buffer()
        .ok_or("Failed to create buffer")
        .unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    unsafe {
        let positions_array_buffer_view = js_sys::Float32Array::view(&positions);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buffer_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let colors = [
        1.0, 1.0, 1.0, 1.0, // White
        1.0, 0.0, 0.0, 1.0, // Red
        0.0, 1.0, 0.0, 1.0, // Green
        0.0, 0.0, 1.0, 1.0, // Blue
    ];
    let color_buffer = context
        .create_buffer()
        .ok_or("Failed to create buffer")
        .unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    unsafe {
        let colors_array_buffer_view = js_sys::Float32Array::view(&colors);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &colors_array_buffer_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    (position_buffer, color_buffer)
}

fn draw_colored_square(
    context: &WebGl2RenderingContext,
    info: &ShaderInfo,
    (position_buffer, color_buffer): &(WebGlBuffer, WebGlBuffer),
) {

    let field_of_view = 45.0 * std::f32::consts::PI / 180.0;
    let aspect = info.canvas_width / info.canvas_height;
    let z_near = 0.1;
    let z_far = 100.0;

    let projection_matrix = glm::perspective(aspect, field_of_view, z_near, z_far);
    let vec_projection_matrix = projection_matrix.iter().map(|v| *v).collect::<Vec<_>>();

    let model_view_matrix =
        glm::translate(&glm::Mat4::identity(), &glm::TVec3::new(-0.0, 0.0, -6.0));
    let vec_model_view_matrix = model_view_matrix.iter().map(|v| *v).collect::<Vec<_>>();

    draw(context, info, position_buffer, color_buffer, &vec_projection_matrix[..], &vec_model_view_matrix[..]);
}
