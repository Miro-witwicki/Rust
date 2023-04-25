use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 30.0;

pub fn to_coordinate(game_coordinate: i32) -> f64 {
    return (game_coordinate as f64) * BLOCK_SIZE;
}
pub fn to_coordinate_u32(game_coordinate: i32) -> u32 {
    return to_coordinate(game_coordinate) as u32;
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coordinate(x);
    let gui_y = to_coordinate(y);
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coordinate(x);
    let y = to_coordinate(y);
    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
