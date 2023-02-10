use js_sys::{Function, Math, Object};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCanvasElement, MouseEvent, WebGl2RenderingContext, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    let canvas: Element = document.query_selector("#canvas-view").unwrap().unwrap();
    let button: Element = document.query_selector("#color-switcher").unwrap().unwrap();

    let canvas: HtmlCanvasElement = canvas.unchecked_into::<HtmlCanvasElement>();
    let context: Object = canvas.get_context("webgl2").unwrap().unwrap();

    let gl: WebGl2RenderingContext = context.unchecked_into::<WebGl2RenderingContext>();

    gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());

    let closure = Closure::<dyn Fn(MouseEvent)>::new(move |_event: MouseEvent| {
        // Get a random color value using a helper function.
        let color = get_random_color();

        // Set the clear color to the random color.
        gl.clear_color(color[0], color[1], color[2], 1.0);

        // Clear the context with the newly set color. This is
        // the function call that actually does the drawing.
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    });

    let listener: &Function = closure.as_ref().unchecked_ref();

    drop(canvas.add_event_listener_with_callback("click", listener));
    drop(button.add_event_listener_with_callback("click", listener));

    closure.forget();

    Ok(())
}

fn get_random_color() -> [f32; 3] {
    let r = Math::random() as f32;
    let g = Math::random() as f32;
    let b = Math::random() as f32;
    [r, g, b]
}
