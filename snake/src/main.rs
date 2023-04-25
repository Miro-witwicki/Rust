extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;

use draw::to_coordinate_u32;
use piston_window::types::Color;
use piston_window::*;

use game::Game;

const BACK_COLOR: Color = [0.3, 0.3, 0.3, 0.3];
fn main() {
    let (width, height) = (20, 20);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coordinate_u32(width), to_coordinate_u32(height)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game: Game = Game::new(width, height);
    while let Some(event)  = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }
        window.draw_2d( &event,|c,g, _d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });

    }

}
