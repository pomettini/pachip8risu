#![no_std]

extern crate alloc;

extern crate playdate as pd;
extern crate playdate_menu;

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
use playdate_menu::*;

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
    pending_state: Rc<RefCell<Option<MyState>>>,
    menu: MyMenu,
    game: MyGame,
}

impl Game for MyMain {
    fn new(pd: &Playdate) -> Self {
        let state = Rc::new(RefCell::new(MyState::Menu));
        let pending_state = Rc::new(RefCell::new(None));

        let mut menu = MyMenu::new(pd);
        let mut game = MyGame::new(pd);

        // Clone for closure
        let pending_state_clone = Rc::clone(&pending_state);
        menu.set_on_state_change(move |new_state| {
            *pending_state_clone.borrow_mut() = Some(new_state);
        });
        let pending_state_clone = Rc::clone(&pending_state);
        game.set_on_state_change(move |new_state| {
            *pending_state_clone.borrow_mut() = Some(new_state);
        });

        Self {
            state,
            pending_state,
            menu,
            game,
        }
    }

    fn update(&mut self, pd: &Playdate) {
        // Handle deferred state change
        if let Some(new_state) = self.pending_state.borrow_mut().take() {
            match new_state {
                MyState::Menu => self.menu.on_enter(),
                MyState::Game(id) => self.game.on_enter(id),
            }
            *self.state.borrow_mut() = new_state;
        }

        // Call update based on current state
        match *self.state.borrow() {
            MyState::Menu => self.menu.update(pd),
            MyState::Game(_) => self.game.update(pd),
        }
    }
}

game_loop!(MyMain);
