extern crate rand;

extern crate piston_window;
extern crate graphics;

use rand::Rng;

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

#[derive(Copy, Clone)]
struct SquarePositions {
    horizontal_position: f64,
    vertical_position: f64,
}

fn display_squares(
    squares: &[Rectangle; 50],
    positions: &[SquarePositions; 50],
    context: &Context,
    window: &mut G2d,
) {

    for (index, square) in squares.iter().enumerate() {
        square.draw(
            [
                positions[index].horizontal_position,
                positions[index].vertical_position,
                10.0,
                10.0
            ],
            &context.draw_state,
            context.transform,
            window
        );
    }
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
        "Sorting algorithms",
        [500, 500]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let squares = [
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

    let mut i = 1;
    let mut j;

    generate_positions(
        &array,
        &mut positions,
    );

    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(Key::Space)) = event.release_args() {

            /* TODO #10 should be refactored: the program must be able
               to handle different algorithms */

            if i == 50 {
                continue;
            }

            j = i;

            while j > 0 && array[j - 1] > array[j] {
                array.swap(j, j - 1);
                j -= 1;
            }

            i += 1;

            generate_positions(
                &array,
                &mut positions,
            );
        }

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [1.0, 1.0, 1.0, 1.0],
                    window
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
