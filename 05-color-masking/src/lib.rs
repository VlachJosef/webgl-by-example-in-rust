use js_sys::{Math, Object};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{
    Element, EventTarget, HtmlCanvasElement, MouseEvent, Node, WebGl2RenderingContext, Window,
};

fn get_random_color() -> [f32; 3] {
    let r = Math::random() as f32;
    let g = Math::random() as f32;
    let b = Math::random() as f32;
    [r, g, b]
}

fn draw_animation(gl: &WebGl2RenderingContext) {
    let color = get_random_color();
    gl.clear_color(color[0], color[1], color[2], 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas: Element = document.query_selector("canvas").unwrap().unwrap();
    let canvas: HtmlCanvasElement = canvas.unchecked_into::<HtmlCanvasElement>();
    let context: Object = canvas.get_context("webgl2").unwrap().unwrap();

    let gl: WebGl2RenderingContext = context.unchecked_into::<WebGl2RenderingContext>();

    gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());

    let gl1 = Rc::new(gl);
    let gl2 = Rc::clone(&gl1);

    let redtoggle: Element = document.query_selector("#red-toggle").unwrap().unwrap();
    let greentoggle: Element = document.query_selector("#green-toggle").unwrap().unwrap();
    let bluetoggle: Element = document.query_selector("#blue-toggle").unwrap().unwrap();

    let greentoggle: Rc<Element> = Rc::new(greentoggle);
    let bluetoggle: Rc<Element> = Rc::new(bluetoggle);

    let greentoggle_closure: Rc<Element> = Rc::clone(&greentoggle);
    let bluetoggle_closure: Rc<Element> = Rc::clone(&bluetoggle);

    let draw_animation_closure = Closure::<dyn Fn()>::new(move || draw_animation(&gl2));
    drop(
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            draw_animation_closure.as_ref().unchecked_ref(),
            1000,
        ),
    );

    let mut mask: [bool; 3] = [true, true, true];

    let set_color_mask_closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        let event_target: EventTarget = event.target().unwrap();

        let greentoggle_event_target: &EventTarget = greentoggle_closure.as_ref();
        let bluetoggle_event_target: &EventTarget = bluetoggle_closure.as_ref();

        let index = if &event_target == greentoggle_event_target {
            1
        } else if &event_target == bluetoggle_event_target {
            2
        } else {
            0
        };

        mask[index] = !mask[index];

        event_target
            .unchecked_into::<Node>()
            .set_text_content(Some(if mask[index] { "On" } else { "Off" }));

        gl1.color_mask(mask[0], mask[1], mask[2], true);

        draw_animation(&gl1);
    });

    let set_color_mask = set_color_mask_closure.as_ref().unchecked_ref();

    drop(redtoggle.add_event_listener_with_callback("click", set_color_mask));
    drop(greentoggle.add_event_listener_with_callback("click", set_color_mask));
    drop(bluetoggle.add_event_listener_with_callback("click", set_color_mask));

    set_color_mask_closure.forget();
    draw_animation_closure.forget();

    Ok(())
}
