use js_sys::{Function, Math, Object};
use std::cell::{Cell, RefCell, RefMut};
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

fn draw_animation(
    gl: &WebGl2RenderingContext,
    size: &[i32; 2],
    velocity: &Rc<Cell<f32>>,
    mut position: RefMut<[i32; 2]>,
    mut color: RefMut<[f32; 3]>,
) {
    gl.scissor(position[0], position[1], size[0], size[1]);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // Every frame the vertical position of the square is
    // decreased, to create the illusion of movement.
    position[1] -= velocity.get() as i32;
    // When the square hits the bottom of the drawing buffer,
    // we override it with new square of different color and
    // velocity.
    if position[1] < 0 {
        // Horizontal position chosen randomly, and vertical
        // position at the top of the drawing buffer.
        position[0] = (Math::random() as f32 * (gl.drawing_buffer_width() - size[0]) as f32) as i32;
        position[1] = gl.drawing_buffer_height();
        // Random velocity between 1.0 and 7.0
        velocity.set(1.0 + 6.0 * Math::random() as f32);
        let new_color = get_random_color();
        color[0] = new_color[0];
        color[1] = new_color[1];
        color[2] = new_color[2];

        gl.clear_color(color[0], color[1], color[2], 1.0);
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let document_start_closure = Rc::new(document);
    let document_stop_closure = Rc::clone(&document_start_closure);

    let gl = Rc::new(get_rendering_context(&document_start_closure));
    let gl2 = Rc::clone(&gl);

    let color: Rc<RefCell<[f32; 3]>> = Rc::new(RefCell::new(get_random_color()));
    let color2: Rc<RefCell<[f32; 3]>> = Rc::clone(&color);

    gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
    gl.clear_color(color.borrow()[0], color.borrow()[1], color.borrow()[2], 1.0);

    // Unlike the browser window, vertical position in WebGL is
    // measured from bottom to top. In here we set the initial
    // position of the square to be at the top left corner of the
    // drawing buffer.
    let position = [0, gl.drawing_buffer_height()];
    let position: Rc<RefCell<[i32; 2]>> = Rc::new(RefCell::new(position));
    let position2: Rc<RefCell<[i32; 2]>> = Rc::clone(&position);

    let size: [i32; 2] = [60, 60];

    let button = document_start_closure
        .query_selector("button")
        .unwrap()
        .unwrap();

    let button1 = Rc::new(button);
    let button2 = Rc::clone(&button1);

    let start_animation_event_handler = Rc::new(RefCell::new(EventListener::new()));
    let stop_animation_event_handler = Rc::new(RefCell::new(EventListener::new()));

    let start_animation_event_handler1 = Rc::clone(&start_animation_event_handler);
    let start_animation_event_handler2 = Rc::clone(&start_animation_event_handler);
    let stop_animation_event_handler1 = Rc::clone(&stop_animation_event_handler);
    let stop_animation_event_handler2 = Rc::clone(&stop_animation_event_handler);

    let timer1: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let timer2: Rc<Cell<Option<i32>>> = Rc::clone(&timer1);

    let velocity: Rc<Cell<f32>> = Rc::new(Cell::new(3.0));
    let velocity2: Rc<Cell<f32>> = Rc::clone(&velocity);

    let draw_animation_closure = Closure::<dyn Fn()>::new(move || {
        draw_animation(
            &gl,
            &size,
            &velocity,
            position.borrow_mut(),
            color.borrow_mut(),
        )
    });

    let start_animation_closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
        document_start_closure
            .query_selector("strong")
            .unwrap()
            .unwrap()
            .set_inner_html("stop");
        drop(button1.remove_event_listener_with_event_listener(
            event.type_().as_str(),
            &start_animation_event_handler1.borrow(),
        ));
        drop(button1.add_event_listener_with_event_listener(
            "click",
            &stop_animation_event_handler1.borrow(),
        ));

        let interval = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                draw_animation_closure.as_ref().unchecked_ref(),
                17,
            );

        timer1.set(Some(interval.unwrap()));

        draw_animation(
            &gl2,
            &size,
            &velocity2,
            position2.borrow_mut(),
            color2.borrow_mut(),
        )
    });

    let stop_animation_closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
        document_stop_closure
            .query_selector("strong")
            .unwrap()
            .unwrap()
            .set_inner_html("start");
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

    start_animation_closure.forget();
    stop_animation_closure.forget();

    Ok(())
}
