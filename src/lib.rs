use game::Game;
use logic::Update;
use ui::draw;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue, JsCast};
use web_sys::{console, HtmlCanvasElement, CanvasRenderingContext2d};

mod ui;
mod paddle;
mod game;
mod logic;

extern crate console_error_panic_hook;
use std::{panic, rc::Rc, thread, sync::Arc, cell::RefCell};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    //alert("Hello, world!");
    let window = web_sys::window().expect("global window does not exist");
    
    let document = window.document().expect("no root document on window");
    let canvas = document.get_element_by_id("maincanvas").expect("no canvas element found");
    let canvas: HtmlCanvasElement = canvas.dyn_into().expect("It wasn't a canvas element");

    let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;
    let context = Rc::new(context);

    let mut game = Game::default();
    let game_ref = Rc::new(game);
    
    draw(context, game_ref);
    
    let closure = Closure::<dyn FnMut()>::new(move||{
        game.tick();
    });
    
    
    window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1000)?;

//        let game = Rc::new(game.clone());//
 //       game.tick();
    //}

    //console::log_1(&context);

    //console::log_1(&"Hello World".into());
    Ok(())
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}