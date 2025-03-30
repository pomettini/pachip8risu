use alloc::boxed::Box;
use pd::{
    graphics::text::TextAlignmentExt,
    sys::ffi::{PDTextAlignment, PDTextWrappingMode},
};

use super::*;

pub struct MyMenu {
    pub on_state_change: Option<Box<dyn FnMut(MyState)>>,
}

impl Game for MyMenu {
    fn new(_: &Playdate) -> Self {
        Self {
            on_state_change: None,
        }
    }

    fn update(&mut self, _pd: &Playdate) {
        System::Cached().draw_fps(0, 0);
        Graphics::Cached()
            .draw_text_in_rect(
                "This is the menu",
                0,
                120 - 8,
                400,
                16,
                PDTextWrappingMode::kWrapClip,
                PDTextAlignment::Center,
            )
            .unwrap();

        let buttons = Buttons::Cached();

        if buttons.pushed().up() {
            if let Some(ref mut callback) = self.on_state_change {
                callback(MyState::Game(1));
            }
        }
    }
}

impl MyMenu {
    pub fn set_on_state_change<F>(&mut self, callback: F)
    where
        F: FnMut(MyState) + 'static,
    {
        self.on_state_change = Some(Box::new(callback));
    }
}
