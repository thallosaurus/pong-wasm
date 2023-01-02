use game::Game;
//use ui::draw;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue, JsCast};
use web_sys::{console, HtmlCanvasElement, CanvasRenderingContext2d};

mod ui;
mod app;
mod paddle;
mod game;
mod ball;

extern crate console_error_panic_hook;
use std::{panic, rc::Rc, thread, sync::{Arc, Mutex}, cell::RefCell};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}