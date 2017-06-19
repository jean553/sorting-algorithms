extern crate rand;

extern crate piston_window;
extern crate graphics;

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

#[derive(Copy, Clone)]
struct SquarePositions {
    horizontal_position: f64,
    vertical_position: f64,
}

fn generate_positions(
    array: &[u8; 50],
    positions: &mut [SquarePositions; 50],
) {

    for index in 0..50 {

        positions[index].horizontal_position =
            (index as f64) * 10.0;

        positions[index].vertical_position =
            490.0 - ((array[index] as f64) * 10.0);
    }
}

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Tic-Tac-Toe",
        [500, 500]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut squares = [
        Rectangle {
            color: [0.0, 0.0, 0.0, 1.0],
            shape: rectangle::Shape::Square,
            border: None,
        }; 50];

    let mut positions = [
        SquarePositions {
            horizontal_position: 0.0,
            vertical_position: 0.0,
        }; 50];

    let mut array: [u8; 50] = [0; 50];

    for value in array.iter_mut() {
        *value = rand::thread_rng().gen_range(1, 41);
    }

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [1.0, 1.0, 1.0, 1.0],
                    window
                );

                generate_positions(
                    &array,
                    &mut positions,
                );

                for (index, square) in squares.iter().enumerate() {

                    let horizontal_position = positions[index].horizontal_position;
                    let vertical_position = positions[index].vertical_position;

                    square.draw(
                        [
                            horizontal_position,
                            vertical_position,
                            10.0,
                            10.0
                        ],
                        &context.draw_state,
                        context.transform,
                        window
                    );
                }
            }
        );
    }
}
