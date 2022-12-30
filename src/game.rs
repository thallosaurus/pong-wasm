use crate::{paddle::Paddle, ui::Drawable, logic::Update};

#[derive(Clone)]
pub struct Game {
    paddles: Vec<Paddle>,
    state: State
}

impl Default for Game {
    fn default() -> Self {
        Self { paddles: vec![
            Player::One.into(),
            Player::Two.into(),
        ], state: State::Stopped }
    }
}

impl Drawable for Game {
    fn draw(&self, ctx: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, area: crate::ui::Rect) {
        ctx.set_fill_style(&"red".into());

        for p in self.paddles.iter() {
            ctx.fill_rect(p.area.x, p.area.y, p.area.w, p.area.h);
        }
    }
}

impl Update for Game {
    fn tick(&mut self) {
        self.paddles[0].area.x += 3.0;
    }
}

#[derive(Clone, Copy)]
enum State {
    Running,
    Paused,

    //Set, when the Game is on the Main Menu
    Stopped
}

pub enum Player {
    One,
    Two
}