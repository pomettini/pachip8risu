#![no_std]

extern crate alloc;

extern crate playdate as pd;

use core::cell::RefCell;

use alloc::rc::Rc;
use crankit_game_loop::{game_loop, Game, Playdate};
use pd::controls::buttons::PDButtonsExt;
use pd::controls::peripherals::Buttons;
use pd::graphics::api::Cache;
use pd::graphics::Graphics;
use pd::println;
use pd::sys::ffi::{LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE};
use pd::system::prelude::*;

pub mod game;
use game::*;

pub mod pachip8risu;
use pachip8risu::*;

pub mod menu;
use menu::*;

#[derive(Debug, Clone, Copy)]
pub enum MyState {
    Menu,
    Game(u8),
}

pub struct MyMain {
    state: Rc<RefCell<MyState>>,
    menu: MyMenu,
    game: MyGame,
}

impl Game for MyMain {
    fn new(pd: &Playdate) -> Self {
        let state = Rc::new(RefCell::new(MyState::Menu));
        let mut menu = MyMenu::new(pd);
        let game = MyGame::new(pd);

        let state_clone = Rc::clone(&state);
        menu.set_on_state_change(move |new_state| {
            *state_clone.borrow_mut() = new_state;
        });

        Self { state, menu, game }
    }

    fn update(&mut self, pd: &Playdate) {
        {
            let current_state = *self.state.borrow();
            match current_state {
                MyState::Menu => {
                    self.menu.update(pd);
                }
                MyState::Game(_) => {
                    self.game.update(pd);
                }
            }
        }
    }
}

game_loop!(MyMain);
