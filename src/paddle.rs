use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};

use crate::{ui::Rect, game::Player, ball::Ball};

#[derive(Clone)]
//#[wasm_bindgen]
pub struct Paddle {
    pub area: Rect,
    speed: f64,
    canvas: HtmlCanvasElement
}

impl Paddle {
    pub fn new(p: Player, canvas: HtmlCanvasElement) -> Self {
 
        let x_1 = 0.0 + 25.0;
        let y_1 = 0.0 + 25.0;

        let x_2 = canvas.width() as f64;
        let x_2 = x_2 - 75.0;

        let y_2 = (canvas.height() / 2) as f64;
        let y_2 = y_2 - 75.0;

        match p {
            Player::One => Self {
                area: Rect {
                    x: x_1,
                    y: y_1,
                    w: 50.0,
                    h: 50.0,
                },
                speed: 5.0,
                canvas,
            },
            Player::Two => Self {
                area: Rect {
                    x: x_2,
                    y: y_2,
                    w: 50.0,
                    h: 50.0,
                },
                speed: 4.0,
                canvas,
            }
        }
    }

    pub fn update_human(&mut self, x: u32, y: u32) {
        //self.area.x = x as f64;
        self.area.y = y as f64;
    }

    pub fn update_ai(&mut self, ball: &Ball) {
        if ball.y < self.area.y {
            self.area.y -= 1.0 * self.speed;
        } else {
            self.area.y += 1.0 * self.speed;
        }
    }
}

enum Movement {
    Top,
    Bottom,
    Left,
    Right
}

impl Movement {
    fn from(start: (f64, f64), end: (f64, f64)) {

        //start.x < end.x - start.y > end.y
        match (start.0 < end.0, start.1 < end.1) {
            (true, true) => {
                //v l n f && v o n u
                console::log_1(&"v l n f && v o n u".into());

            },
            (true, false) => todo!(),
            (false, true) => todo!(),
            (false, false) => todo!(),
        }
    }
}