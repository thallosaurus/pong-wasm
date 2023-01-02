use std::f32::consts::TAU;

use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d};

use crate::{
    ball::Ball,
    paddle::Paddle, ui::Rect,
    //ui::{Drawable, Rect},
};

//#[derive(Clone)]
#[wasm_bindgen]
pub struct Game {
    paddles: Vec<Paddle>,
    ball: Ball,
    state: State,
}

#[wasm_bindgen]
impl Game {
    pub fn new(ctx: &CanvasRenderingContext2d) -> Self {
        let canvas = ctx.canvas();
        let canvas = match canvas {
            Some(c) => c,
            None => panic!("Game: Couldn't get canvas"),
        };

        let canvas_a = canvas.clone();
        let canvas_b = canvas.clone();

        let ball = Ball::new(canvas);

        Self {
            paddles: vec![Paddle::new(Player::One, canvas_a), Paddle::new(Player::Two, canvas_b)],
            state: State::Stopped,
            ball
        }
    }

    pub fn update_tick(&mut self) {
        //self.paddles[0].area.x += 3.0;
        self.ball.update();
        self.update_enemy();
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, area: Rect) {
        for paddle in &self.paddles {
            let a = paddle.area;
            ctx.set_fill_style(&"white".into());
            ctx.fill_rect(a.x, a.y, a.w, a.h);
        }

        //arc(x, y, radius, startAngle, endAngle)
        ctx.set_stroke_style(&"white".into());
        ctx.begin_path();
        ctx.arc(self.ball.x, self.ball.y, self.ball.radius, 0.0, TAU.into());
        ctx.stroke();

        ctx.set_fill_style(&"white".into());
        ctx.fill();
    }

    fn update_enemy(&mut self) {
        let mut enemy = self.paddles.get_mut(1).unwrap();
        enemy.update_ai(&self.ball);
    }
    
    pub fn mouse_input(&mut self, x: u32, y: u32) {
        let mut human = self.paddles.get_mut(0).unwrap();
        human.update_human(x, y);
    }
}

#[derive(Clone, Copy)]
enum State {
    Running,
    Paused,

    //Set, when the Game is on the Main Menu
    Stopped,
}

pub enum Player {
    One,
    Two,
}
