use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::console;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::game::Game;
use crate::ui::Rect;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub struct App {
    ctx: CanvasRenderingContext2d,
    game: Game,
}

#[wasm_bindgen]
impl App {

    #[wasm_bindgen(constructor)]
    pub fn new_app(ctx: CanvasRenderingContext2d) -> App {
        let mut game = Game::new(&ctx);

        Self { ctx, game }
    }

    pub fn draw_tick(&self) {
        //let f = Rc::new(RefCell::new(None));
        //let g = f.clone();

        //*g.borrow_mut() = Some(Closure::new(move || {
        //let ctx = ctx.clone();
        let canvas = self.ctx.canvas().unwrap();

        self.ctx
            .clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        self.draw_background(&canvas, &self.ctx);
        let area = Rect {
            x: 0.0,
            y: 0.0,
            w: canvas.width().into(),
            h: canvas.height().into(),
        };

        self.ctx.set_font("bold 48px serif");

        self.ctx.set_stroke_style(&"white".into());

        //self.ctx.stroke_text(&"Hello World", 100.0, 100.0);

        self.game.draw(&self.ctx, area.clone());

        //for paddle in self.game.
    }

    pub fn update_tick(&mut self) {
        self.game.update_tick();
    }

    fn draw_background(&self, canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&self.game.get_background().into());
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    }

    pub fn mouse_move_input(&mut self, x: u32, y: u32) {
        self.game.mouse_input(x,y);
    }
}
pub trait Drawable {
    fn draw(&self, ctx: &Rc<CanvasRenderingContext2d>, area: Rect);
}

mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::ui::Rect;

    #[cfg(test)]
    fn configure() {
        use wasm_bindgen_test::wasm_bindgen_test_configure;

        wasm_bindgen_test_configure!(run_in_browser);
    }

    #[wasm_bindgen_test]
    fn rect1_contains_rect2() {
        let rect1 = Rect {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 10.0,
        };
        let rect2 = Rect {
            x: 2.0,
            y: 2.0,
            w: 5.0,
            h: 5.0,
        };

        assert!(rect1.contains(rect2));
    }

    #[wasm_bindgen_test]
    fn rect1_doesnt_contain_rect2() {
        let rect1 = Rect {
            x: 7.0,
            y: 7.0,
            w: 10.0,
            h: 10.0,
        };
        let rect2 = Rect {
            x: 2.0,
            y: 2.0,
            w: 5.0,
            h: 5.0,
        };

        assert!(!rect1.contains(rect2));
    }

    #[wasm_bindgen_test]
    fn rect1_cuts_rect2() {
        let rect1 = Rect {
            x: 5.0,
            y: 5.0,
            w: 10.0,
            h: 10.0,
        };
        let rect2 = Rect {
            x: 2.0,
            y: 2.0,
            w: 5.0,
            h: 5.0,
        };

        assert!(!rect1.contains(rect2));
    }
}
