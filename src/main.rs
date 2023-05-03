extern crate mchip8;
extern crate ruscii;

use mchip8::Chip8;
use std::{fs::File, io::Read};

use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;
use std::time::{SystemTime, UNIX_EPOCH};

const WIDTH: i32 = 64;
const HEIGHT: i32 = 32;

fn main() {
    let mut file = File::open("roms/maze.rom").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();

    let mut cpu = Chip8::new();
    cpu.load_rom(&buf, Some(10));
    cpu.set_random_seed(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let mut fps_counter = FPSCounter::default();
    let mut app = App::config(Config::new().fps(60));

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            if let KeyEvent::Pressed(Key::Q | Key::Esc) = key_event {
                app_state.stop();
            }
        }

        fps_counter.update();

        cpu.update();

        let mut pencil = Pencil::new(window.canvas_mut());

        (0..WIDTH * HEIGHT).for_each(|p| {
            let x = p % 64;
            let y = 2 + p / 64;
            if cpu.draw_unoptimized()[p as usize] {
                pencil.draw_text("█", Vec2::xy(x, y));
            }
        });

        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 0));
    });
}
