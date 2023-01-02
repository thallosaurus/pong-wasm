use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

use crate::paddle::{self, Paddle};

#[derive(Clone)]
//#[wasm_bindgen]
pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    canvas: HtmlCanvasElement,
    dir: (i32, i32),
    speed: f64,
}

impl Ball {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let x = canvas.width() / 2;
        let y = canvas.height() / 2;

        let radius = 5.0;

        Self {
            x: x.into(),
            y: y.into(),
            radius: radius,
            canvas,
            dir: (1, 1),
            speed: 10.0,
        }
    }

    pub fn update(&mut self) -> bool {
        let old_dir = self.dir;

        let dir_x = self.update_x();
        let dir_y = self.update_y();

        let s = format!("{:?}, {:?}", (dir_x as i32, dir_y as i32), old_dir);

        self.x += dir_x * self.speed;
        self.y += dir_y * self.speed;

        old_dir != (dir_x as i32, dir_y as i32)
    }

    fn update_x(&mut self) -> f64 {
        match (
            self.x - self.radius > 0.0,
            self.x + self.radius < self.canvas.width().into(),
        ) {
            (true, true) => {}
            (true, false) => {
                self.flipX();
            }
            (false, true) => {
                self.flipX();
            }
            (false, false) => {
                self.flipX();
            }
        }

        self.dir.0.into()
    }

    fn update_y(&mut self) -> f64 {
        match (
            self.y - self.radius > 0.0,
            self.y + self.radius < self.canvas.height().into(),
        ) {
            (true, true) => {}
            (true, false) => {
                self.flipY();
            }
            (false, true) => {
                self.flipY();
            }
            (false, false) => {
                self.flipX();
            }
        }

        self.dir.1.into()
    }

    pub fn check_collision_with_paddles(&mut self, paddles: Vec<Paddle>) {}

    pub fn flipX(&mut self) {
        self.dir.0 *= -1;
    }

    pub fn flipY(&mut self) {
        self.dir.1 *= -1;
    }
}
