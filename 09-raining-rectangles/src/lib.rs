use js_sys::{Math, Object};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{
    Document, Element, HtmlCanvasElement, HtmlElement, MouseEvent, Node, WebGl2RenderingContext,
    Window,
};

struct Rectangle {
    size: [i32; 2],
    position: [i32; 2],
    velocity: i32,
}

struct GameState {
    score: i32,
    misses: i32,
    score_display: Node,
    misses_display: Node,
}

impl GameState {
    fn new(document: &Document) -> GameState {
        let strongs = document.query_selector_all("strong").unwrap();
        let score = 0;
        let misses = 0;
        let score_display = strongs.get(0).unwrap();
        let misses_display = strongs.get(1).unwrap();
        GameState {
            score,
            misses,
            score_display,
            misses_display,
        }
    }

    fn hit(&mut self) {
        self.score += 1;
        self.score_display
            .set_text_content(Some(&self.score.to_string()));
    }
    fn miss(&mut self) {
        self.misses += 1;
        self.misses_display
            .set_text_content(Some(&self.misses.to_string()));
    }
}

impl Rectangle {
    fn initiate(gl: &WebGl2RenderingContext) -> ([i32; 2], [i32; 2], i32, [f32; 3]) {
        let rand_nums = get_random_vector();
        let size = [
            (5.0 + 120.0 * rand_nums[0]) as i32,
            (5.0 + 120.0 * rand_nums[1]) as i32,
        ];
        let position = [
            (rand_nums[2] * (gl.drawing_buffer_width() - size[0]) as f32) as i32,
            gl.drawing_buffer_height(),
        ];
        let velocity = (1.0 + 6.0 * Math::random() as f32) as i32;
        let color = get_random_vector();

        (size, position, velocity, color)
    }

    fn new(gl: &WebGl2RenderingContext) -> Self {
        let (size, position, velocity, color) = Self::initiate(gl);
        gl.clear_color(color[0], color[1], color[2], 1.0);
        Self {
            size,
            position,
            velocity,
        }
    }

    fn regenerate(&mut self, gl: &WebGl2RenderingContext) {
        let (size, position, velocity, color) = Self::initiate(gl);
        self.size = size;
        self.position = position;
        self.velocity = velocity;
        gl.clear_color(color[0], color[1], color[2], 1.0);
    }
}

fn get_random_vector() -> [f32; 3] {
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
    mut raining_rect: RefMut<Rectangle>,
    mut game_state: RefMut<GameState>,
) {
    gl.scissor(
        raining_rect.position[0],
        raining_rect.position[1],
        raining_rect.size[0],
        raining_rect.size[1],
    );
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // Every frame the vertical position of the square is
    // decreased, to create the illusion of movement.
    raining_rect.position[1] -= raining_rect.velocity;

    // When the square hits the bottom of the drawing buffer,
    // we override it with new square of different color and
    // velocity.
    if raining_rect.position[1] < 0 {
        game_state.miss();
        raining_rect.regenerate(gl);
    }
    // We are using setTimeout for animation. So we reschedule
    // the timeout to call drawAnimation again in 17ms.
    // Otherwise we won't get any animation.
    // timer = setTimeout(drawAnimation, 17);
}

fn player_click(
    gl: &WebGl2RenderingContext,
    evt: MouseEvent,
    mut raining_rect: RefMut<Rectangle>,
    mut game_state: RefMut<GameState>,
) {
    // We need to transform the position of the click event from
    // window coordinates to relative position inside the canvas.
    // In addition we need to remember that vertical position in
    // WebGL increases from bottom to top, unlike in the browser
    // window.
    let target: HtmlElement = evt.target().unwrap().unchecked_into::<HtmlElement>();
    let position = [
        evt.page_x() - target.offset_left(),
        gl.drawing_buffer_height() - (evt.page_y() - target.offset_top()),
    ];
    // If the click falls inside the rectangle, we caught it.

    // Increment score and create a new rectangle.
    let diff_pos = [
        position[0] - raining_rect.position[0],
        position[1] - raining_rect.position[1],
    ];
    if diff_pos[0] >= 0
        && diff_pos[0] < raining_rect.size[0]
        && diff_pos[1] >= 0
        && diff_pos[1] < raining_rect.size[1]
    {
        game_state.hit();
        raining_rect.regenerate(gl);
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let game_state = Rc::new(RefCell::new(GameState::new(&document)));
    let game_state2 = Rc::clone(&game_state);

    let document_start_closure = Rc::new(&document);
    //let document_stop_closure = Rc::clone(&document_start_closure);

    let gl = Rc::new(get_rendering_context(&document_start_closure));
    let gl2 = Rc::clone(&gl);

    gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
    let raining_rect = Rc::new(RefCell::new(Rectangle::new(&gl)));
    let raining_rect2 = Rc::clone(&raining_rect);

    let draw_animation_closure = Closure::<dyn Fn()>::new(move || {
        draw_animation(&gl, raining_rect.borrow_mut(), game_state.borrow_mut())
    });

    drop(
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            draw_animation_closure.as_ref().unchecked_ref(),
            17,
        ),
    );

    let player_click_closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        player_click(
            &gl2,
            event,
            raining_rect2.borrow_mut(),
            game_state2.borrow_mut(),
        );
    });

    let player_click_function = player_click_closure.as_ref().unchecked_ref();

    drop(
        document
            .query_selector("canvas")
            .unwrap()
            .unwrap()
            .add_event_listener_with_callback("click", player_click_function),
    );

    draw_animation_closure.forget();
    player_click_closure.forget();

    Ok(())
}
