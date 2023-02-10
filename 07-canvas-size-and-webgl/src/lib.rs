use js_sys::Object;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlCollection, WebGl2RenderingContext, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvases: HtmlCollection = document.get_elements_by_tag_name("canvas");
    let first_canvas = canvases
        .item(0)
        .unwrap()
        .unchecked_into::<HtmlCanvasElement>();
    let second_canvas = canvases
        .item(1)
        .unwrap()
        .unchecked_into::<HtmlCanvasElement>();

    first_canvas.set_width(first_canvas.client_width() as u32);
    first_canvas.set_height(first_canvas.client_height() as u32);

    vec![first_canvas, second_canvas].iter().for_each(|canvas| {
        let context: Object = canvas.get_context("webgl2").unwrap().unwrap();
        let gl: WebGl2RenderingContext = context.unchecked_into::<WebGl2RenderingContext>();

        gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
        gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
        gl.scissor(30, 10, 60, 60);
        gl.clear_color(1.0, 1.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    });

    Ok(())
}
