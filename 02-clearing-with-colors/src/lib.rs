use js_sys::Object;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCanvasElement, WebGl2RenderingContext, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    let paragraph: Element = document.query_selector("p").unwrap().unwrap();
    let canvas: Element = document.query_selector("canvas").unwrap().unwrap();

    let canvas: HtmlCanvasElement = canvas.unchecked_into::<HtmlCanvasElement>();
    let context: Object = canvas.get_context("webgl2").unwrap().unwrap();

    let gl: WebGl2RenderingContext = context.unchecked_into::<WebGl2RenderingContext>();

    paragraph.set_inner_html(&format!("Congratulations! Your browser supports WebGL2.",));

    gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());

    // Set the clear color to darkish green.
    gl.clear_color(0.0, 0.5, 0.0, 1.0);

    // Clear the context with the newly set color. This is
    // the function call that actually does the drawing.
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}
