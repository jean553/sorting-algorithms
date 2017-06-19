extern crate piston_window;
extern crate graphics;

use std::mem;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
    Rectangle,
};

use graphics::{
    rectangle,
};

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Tic-Tac-Toe",
        [500, 500]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let test = [
        Rectangle {
            color: [1.0, 1.0, 1.0, 1.0],
            shape: rectangle::Shape::Square,
            border: None,
        }; 50];

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [1.0, 1.0, 1.0, 1.0],
                    window
                );
            }
        );
    }
}
