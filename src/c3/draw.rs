use piston_window::{Rectangle, Context, G2d, rectangle};
use piston_window::types::Color;
use crate::c3::BLOCK_SIZE;

fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}


fn draw_block(color: Color, x: i32, y: i32, context: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        g,
    )
}


fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, context: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        context.transform,
        g,
    )
}