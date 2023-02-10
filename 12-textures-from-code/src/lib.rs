use js_sys::Object;
use std::rc::Rc;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{
    Document, Element, HtmlCanvasElement, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, Window,
};

fn get_rendering_context(document: &Document) -> WebGl2RenderingContext {
    let canvas: Element = document.query_selector("canvas").unwrap().unwrap();
    let canvas: HtmlCanvasElement = canvas.unchecked_into::<HtmlCanvasElement>();
    let context: Object = canvas.get_context("webgl2").unwrap().unwrap();

    // The following two lines set the size (in CSS pixels) of
    // the drawing buffer to be identical to the size of the
    // canvas HTML element, as determined by CSS.
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);

    let gl: WebGl2RenderingContext = context.unchecked_into::<WebGl2RenderingContext>();

    gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let gl = Rc::new(get_rendering_context(&document));
    let gl2 = Rc::clone(&gl);

    let vertex_shader = gl
        .create_shader(WebGl2RenderingContext::VERTEX_SHADER)
        .unwrap();
    gl.shader_source(&vertex_shader, include_str!("shader.vert"));
    gl.compile_shader(&vertex_shader);

    let fragment_shader = gl
        .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
        .unwrap();
    gl.shader_source(&fragment_shader, include_str!("shader.frag"));
    gl.compile_shader(&fragment_shader);
    let program = gl.create_program().unwrap();

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);

    gl.link_program(&program);
    gl.detach_shader(&program, &vertex_shader);
    gl.detach_shader(&program, &fragment_shader);
    gl.delete_shader(Some(&vertex_shader));
    gl.delete_shader(Some(&fragment_shader));

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        gl.enable_vertex_attrib_array(0);
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&[0.0, 0.0]);

            gl2.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        gl.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        gl.use_program(Some(&program));
        gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
        clean_up(&gl, Some(&program), Some(&buffer))
    } else {
        clean_up(&gl, Some(&program), None)
    }

    Ok(())
}

fn clean_up(
    gl: &WebGl2RenderingContext,
    program: Option<&WebGlProgram>,
    buffer: Option<&WebGlBuffer>,
) {
    gl.use_program(None);
    gl.delete_buffer(buffer);
    gl.delete_program(program);
}
