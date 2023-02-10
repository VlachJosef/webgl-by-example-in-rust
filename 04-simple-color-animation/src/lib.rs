use js_sys::{Function, Math, Object};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{
    Document, Element, EventListener, HtmlCanvasElement, MouseEvent, PointerEvent,
    WebGl2RenderingContext, Window,
};

fn get_random_color() -> [f32; 3] {
    let r = Math::random() as f32;
    let g = Math::random() as f32;
    let b = Math::random() as f32;
    [r, g, b]
}

fn draw_animation(gl: &WebGl2RenderingContext) {
    // Get a random color value using a helper function.
    let color = get_random_color();

    // Set the clear color to the random color.
    gl.clear_color(color[0], color[1], color[2], 1.0);

    // Clear the context with the newly set color. This is
    // the function call that actually does the drawing.
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    let button: Element = document
        .query_selector("#animation-onoff")
        .unwrap()
        .unwrap();

    let verb = document.query_selector("strong").unwrap().unwrap();

    let button1: Rc<Element> = Rc::new(button);
    let button2 = Rc::clone(&button1);

    let verb1: Rc<Element> = Rc::new(verb);
    let verb2 = Rc::clone(&verb1);

    let canvas: Element = document.query_selector("#canvas-view").unwrap().unwrap();
    let canvas: HtmlCanvasElement = canvas.unchecked_into::<HtmlCanvasElement>();
    let context: Object = canvas.get_context("webgl2").unwrap().unwrap();

    let gl: WebGl2RenderingContext = context.unchecked_into::<WebGl2RenderingContext>();

    gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());

    let gl1 = Rc::new(gl);
    let gl2 = Rc::clone(&gl1);

    let draw_animation_closure = Closure::<dyn Fn()>::new(move || draw_animation(&gl2));

    let timer1: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let timer2: Rc<Cell<Option<i32>>> = Rc::clone(&timer1);

    let start_animation_event_handler = Rc::new(RefCell::new(EventListener::new()));
    let stop_animation_event_handler = Rc::new(RefCell::new(EventListener::new()));

    let start_animation_event_handler1 = Rc::clone(&start_animation_event_handler);
    let start_animation_event_handler2 = Rc::clone(&start_animation_event_handler);
    let stop_animation_event_handler1 = Rc::clone(&stop_animation_event_handler);
    let stop_animation_event_handler2 = Rc::clone(&stop_animation_event_handler);

    let start_animation_closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
        verb1.set_text_content(Some("stop"));
        drop(button1.remove_event_listener_with_event_listener(
            event.type_().as_str(),
            &start_animation_event_handler1.borrow(),
        ));
        drop(button1.add_event_listener_with_event_listener(
            "click",
            &stop_animation_event_handler1.borrow(),
        ));

        // Setup animation loop by redrawing every second
        let interval = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                draw_animation_closure.as_ref().unchecked_ref(),
                1000,
            );

        timer1.set(Some(interval.unwrap()));

        // Give immediate feedback to user after clicking, by
        // drawing one animation frame.
        draw_animation(&gl1);
    });

    let stop_animation_closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
        verb2.set_text_content(Some("start"));

        drop(button2.remove_event_listener_with_event_listener(
            event.type_().as_str(),
            &stop_animation_event_handler2.borrow(),
        ));
        drop(button2.add_event_listener_with_event_listener(
            "click",
            &start_animation_event_handler2.borrow(),
        ));

        if let Some(handle) = timer2.take() {
            web_sys::window()
                .unwrap()
                .clear_interval_with_handle(handle)
        }
    });

    let start_animation_function: &Function = start_animation_closure.as_ref().unchecked_ref();

    start_animation_event_handler
        .borrow_mut()
        .handle_event(start_animation_function);

    let stop_animation_function: &Function = stop_animation_closure.as_ref().unchecked_ref();

    stop_animation_event_handler
        .borrow_mut()
        .handle_event(stop_animation_function);

    let pointer_event: PointerEvent = PointerEvent::new("click").unwrap();

    drop(stop_animation_function.call1(&JsValue::UNDEFINED, pointer_event.as_ref()));

    // draw_animation_closure.forget(); // Cannot forget, due to move to start_animation_closure closure
    start_animation_closure.forget();
    stop_animation_closure.forget();

    Ok(())
}
