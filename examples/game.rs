// use std::time::{Duration, Instant};
use rust_webgl::*;

fn main() {
    // set_event_handler(move |key| match key {
    //     Key::Left => clear_screen_to_color(1.0, 0.0, 0.0, 1.0),
    //     Key::Right => clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
    //     Key::Up => clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
    //     Key::Down => clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
    //     Key::Space => clear_screen_to_color(1.0, 1.0, 0.0, 1.0),
    // });
    let mut x_position = 0.;
    let mut y_position = 0.;
    let mut x_direction = 1.0;
    let mut y_direction = 1.0;
    let speed = 5.0;

    let mut color = (0.0, 1.0, 0.0);

    set_event_handler(move |context, event| {
        match event {
            Event::KeyDown(key) => {
                match key {
                    Key::Left => color = (0.7, 0.0, 0.0),
                    Key::Right => color = (0.0, 1.0, 0.0),
                    Key::Up => color = (0.0, 0.0, 1.0),
                    Key::Down => color = (0.0, 1.0, 1.0),
                    Key::Space => color = (1.0, 1.0, 0.0),
                    // Key::Right => context.clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
                    // Key::Up => context.clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
                    // Key::Down => context.clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
                    // Key::Space => context.clear_screen_to_color(1.0, 1.0, 0.0, 1.0),
                }
            }
            Event::Draw => {
                x_position += x_direction * speed;
                y_position += y_direction * speed;
                // Change the horizontal direction if the cube's too far to the left or right.
                let cube_dimension = 50.;
                let dims = get_dimensions();
                if x_position <= 0.0 || x_position >= dims.0 as f32 - cube_dimension {
                    x_direction *= -1.0;
                }
                // Change the vertical direction if the cube's too far to the top or bottom.
                if y_position <= 0.0 || y_position >= dims.1 as f32 - cube_dimension {
                    y_direction *= -1.0;
                }
                context.clear_screen_to_color(color.0, color.1, color.2, 1.0);
                context.draw_rectangle(
                    x_position,
                    y_position,
                    cube_dimension,
                    cube_dimension,
                    1.0,
                    0.0,
                    0.0,
                    1.0,
                );
            }
        }
    })
}

// fn main() {
//     let colors = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)];

//     let duration = Duration::from_secs(5);
//     let start_time = Instant::now();

//     loop {
//         let elapsed_time = start_time.elapsed();
//         let progress = elapsed_time.as_secs_f32() / duration.as_secs_f32();

//         let color_index = (progress * (colors.len() - 1) as f32).floor() as usize;
//         let color_progress = progress * (colors.len() - 1) as f32 - color_index as f32;

//         let current_color =
//             interpolate(colors[color_index], colors[color_index + 1], color_progress);
//         println!("Color: {:?}", current_color);
//         // clear_screen_to_color(
//         //     current_color.0,
//         //     current_color.1,
//         //     current_color.2,
//         //     current_color.3,
//         // );
//     }
// }

// fn interpolate(
//     color1: (f32, f32, f32),
//     color2: (f32, f32, f32),
//     progress: f32,
// ) -> (f32, f32, f32, f32) {
//     let r = color1.0 + (color2.0 - color1.0) * progress;
//     let g = color1.1 + (color2.1 - color1.1) * progress;
//     let b = color1.2 + (color2.2 - color1.2) * progress;

//     (r, g, b, 1.0)
// }
