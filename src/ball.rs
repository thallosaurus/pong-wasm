use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

use crate::paddle::{Paddle, self};

#[derive(Clone)]
//#[wasm_bindgen]
pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    canvas: HtmlCanvasElement,
    dir: (i32, i32),
    speed: f64
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
            speed: 10.0
        }
    }

    pub fn update(&mut self) {
        let console_str = format!(
            "x: {}, y: {}, c.w: {}, c.h: {}, dir: ({}, {})",
            self.x,
            self.y,
            self.canvas.width(),
            self.canvas.height(),
            self.dir.0,
            self.dir.1,
        );
        //console::log_1(&console_str.into());
        //console::log_1(&console_str.into());
        
        self.updateX();
        self.updateY();
        /*if self.dir.1 > 0 {
            self.y -= 1.0;
        } else {
            self.y -= 1.0;
        }*/
    }

    fn updateX(&mut self) {
        match (self.x - self.radius > 0.0, self.x + self.radius < self.canvas.width().into()) {
            (true, true) => {
            }
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
        if self.dir.0 < 0 {
            self.x -= 1.0 * self.speed;
        } else {
            self.x += 1.0 * self.speed;
        }
    }
    
    fn updateY(&mut self) {
        match (self.y - self.radius > 0.0, self.y + self.radius < self.canvas.height().into()) {
            (true, true) => {

            }
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
        if self.dir.1 < 0 {
            self.y -= 1.0 * self.speed;
        } else {
            self.y += 1.0 + self.speed;
        }
    }

    pub fn check_collision_with_paddles(&mut self, paddles: Vec<Paddle>) {
        for paddle in paddles {
            
        }
    }

    pub fn flipX(&mut self) {
        self.dir.0 *= -1;
    }

    pub fn flipY(&mut self) {
        self.dir.1 *= -1;
    }
}
