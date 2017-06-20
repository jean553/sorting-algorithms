//! Main file

extern crate rand;

extern crate piston_window;
extern crate graphics;

use rand::Rng;
use std::env;

use piston_window::{
    G2d,
    PistonWindow,
    WindowSettings,
    clear,
    Rectangle,
    Button,
    Key,
    ReleaseEvent,
};

use graphics::{
    Context,
    rectangle,
};

mod insertion_sort;
mod selection_sort;

#[derive(Copy, Clone)]
struct SquarePositions {
    horizontal_position: f64,
    vertical_position: f64,
}

enum Algorithm {
    Insertion,
    Selection,
}

const SQUARE_DIMENSIONS: f64 = 10.0;
const ARRAY_LENGTH: usize = 50;

/// Displays every square contained by the `squares` array;
/// the positions of the squares are wrapped into `positions` array;
/// the method uses the given Piston context and window
fn display_squares(
    squares: &[Rectangle; ARRAY_LENGTH],
    positions: &[SquarePositions; ARRAY_LENGTH],
    context: &Context,
    window: &mut G2d,
) {

    for (index, square) in squares.iter().enumerate() {
        square.draw(
            [
                positions[index].horizontal_position,
                positions[index].vertical_position,
                SQUARE_DIMENSIONS,
                SQUARE_DIMENSIONS,
            ],
            &context.draw_state,
            context.transform,
            window,
        );
    }
}

/// Sets the position of each square contained into `squares`;
/// updates the given `positions` array
fn generate_positions(
    array: &[u8; ARRAY_LENGTH],
    positions: &mut [SquarePositions; ARRAY_LENGTH],
) {

    for index in 0..ARRAY_LENGTH {

        positions[index].horizontal_position =
            (index as f64) * SQUARE_DIMENSIONS;

        let value = array[index];
        const LOW_VERTICAL_POSITION: f64 = 490.0;
        positions[index].vertical_position =
            LOW_VERTICAL_POSITION - ((value as f64) * SQUARE_DIMENSIONS);
    }
}

fn main() {

    let args: Vec<_> = env::args().collect();
    let algorithm = if args.get(1).unwrap() == "insertion" {
        Algorithm::Insertion
    } else {
        Algorithm::Selection
    };

    const WINDOW_WIDTH: u32 = 500;
    const WINDOW_HEIGHT: u32 = 500;
    let mut window: PistonWindow = WindowSettings::new(
        "Sorting algorithms",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let squares = [
        Rectangle {
            color: [0.0, 0.0, 0.0, 1.0], /* black */
            shape: rectangle::Shape::Square,
            border: None,
        }; ARRAY_LENGTH];

    let mut positions = [
        SquarePositions {
            horizontal_position: 0.0,
            vertical_position: 0.0,
        }; ARRAY_LENGTH];

    let mut array: [u8; ARRAY_LENGTH] = [0; ARRAY_LENGTH];

    const MIN_RANDOM_VALUE: u8 = 1;
    const MAX_RANDOM_VALUE: u8 = 41;
    for value in array.iter_mut() {
        *value = rand::thread_rng().gen_range(
            MIN_RANDOM_VALUE,
            MAX_RANDOM_VALUE,
        );
    }

    generate_positions(
        &array,
        &mut positions,
    );

    /* used by every algorithms */
    let mut first_index: usize = 1;
    let mut second_index: usize = 0;

    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(Key::Space)) = event.release_args() {

            /* TODO #10 should be refactored: the program must be able
               to handle different algorithms */

            if first_index == ARRAY_LENGTH {
                continue;
            }

            match algorithm {
                Algorithm::Insertion => {
                    insertion_sort::iterate_over_insertion_sort(
                        &mut array,
                        &mut first_index,
                        &mut second_index,
                    );
                }
                _ => {
                    selection_sort::iterate_over_selection_sort(
                        &mut array,
                        &mut second_index,
                    );
                }
            };

            generate_positions(
                &array,
                &mut positions,
            );

        }

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [1.0, 1.0, 1.0, 1.0], /* white */
                    window,
                );

                display_squares(
                    &squares,
                    &positions,
                    &context,
                    window,
                );
            }
        );
    }
}
