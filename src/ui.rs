use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::game::Game;
use crate::logic::Update;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


pub fn draw(ctx: Rc<CanvasRenderingContext2d>, game: Rc<Game>) {

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    
    *g.borrow_mut() = Some(Closure::new(move || {
        let ctx = ctx.clone();
        let canvas = ctx.canvas().unwrap();

        ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        draw_background(&canvas, &ctx);
        let area = Rect { x: 0.0, y: 0.0, w: canvas.width().into(), h: canvas.height().into() };
        game.draw(&ctx, area);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

}

fn draw_background(canvas: &HtmlCanvasElement, ctx: &Rc<CanvasRenderingContext2d>) {
    ctx.set_fill_style(&"black".into());
    ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
}

pub trait Drawable {
    fn draw(&self, ctx: &Rc<CanvasRenderingContext2d>, area: Rect);
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) w: f64,
    pub(crate) h: f64
}

impl Rect {
    fn contains(&self, other: Self) -> bool {
        self.x < other.x && self.x + self.w > other.x + other.w
            && self.y < other.y && self.y + self.h > other.y + other.h
    }
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
        let rect1 = Rect { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
        let rect2 = Rect { x: 2.0, y: 2.0, w: 5.0, h: 5.0 };
        
        assert!(rect1.contains(rect2));
    }

    #[wasm_bindgen_test]
    fn rect1_doesnt_contain_rect2() {
        let rect1 = Rect { x: 7.0, y: 7.0, w: 10.0, h: 10.0 };
        let rect2 = Rect { x: 2.0, y: 2.0, w: 5.0, h: 5.0 };
        
        assert!(!rect1.contains(rect2));
    }

    #[wasm_bindgen_test]
    fn rect1_cuts_rect2() {
        let rect1 = Rect { x: 5.0, y: 5.0, w: 10.0, h: 10.0 };
        let rect2 = Rect { x: 2.0, y: 2.0, w: 5.0, h: 5.0 };
        
        assert!(!rect1.contains(rect2));
    }
}