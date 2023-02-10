use js_sys::Object;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement, WebGl2RenderingContext, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document = window.document().unwrap();

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

    // Enable scissoring operation and define the position and
    // size of the scissoring area.
    gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
    gl.scissor(40, 20, 60, 130);

    // Clear the drawing buffer solid yellow.
    gl.clear_color(1.0, 1.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}
