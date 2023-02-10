use js_sys::{Function, Object};
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCanvasElement, MouseEvent, WebGl2RenderingContext, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();
    let paragraph: Element = document.query_selector("p").unwrap().unwrap();
    let button: Element = document.query_selector("button").unwrap().unwrap();

    let closure = Closure::<dyn Fn(MouseEvent)>::new(move |_event: MouseEvent| {
        let canvas: Element = document.create_element("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.unchecked_into::<HtmlCanvasElement>();
        let context: Object = canvas.get_context("webgl2").unwrap().unwrap();

        if context.has_type::<WebGl2RenderingContext>() {
            paragraph.set_inner_html("Congratulations! Your browser supports WebGL2.");
        } else {
            paragraph.set_inner_html("Failed. Your browser or device may not support WebGL.");
        }
    });

    let listener: &Function = closure.as_ref().unchecked_ref();

    drop(button.add_event_listener_with_callback("click", listener));

    closure.forget();

    Ok(())
}
