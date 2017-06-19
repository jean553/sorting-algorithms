extern crate rand;

extern crate piston_window;
extern crate graphics;

use std::mem;

use rand::Rng;

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

    let squares = [
        Rectangle {
            color: [1.0, 0.0, 1.0, 1.0],
            shape: rectangle::Shape::Square,
            border: None,
        }; 50];

    let mut array: [u8; 50] = [0; 50];

    for value in array.iter_mut() {
        *value = rand::thread_rng().gen_range(1, 101);
    }

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [1.0, 1.0, 1.0, 1.0],
                    window
                );

                for square in squares.iter() {

                    square.draw(
                        [10.0, 10.0, 20.0, 20.0],
                        &context.draw_state,
                        context.transform,
                        window
                    );
                }
            }
        );
    }
}
