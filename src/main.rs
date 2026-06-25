mod color;
mod complex;
mod fractal;
mod view;

use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    color::FractalColorType,
    complex::C64,
    fractal::{Fractal, Julia},
    view::{ComplexView, Dimension},
};

const ZOOM_RATE: f64 = 2.; // # of doubles per second
const PAN_RATE: f64 = 2.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_owned(),
        window_width: 800,
        window_height: 800,
        // fullscreen: true,
        ..Default::default()
    }
}

fn update_fractal_texture<T: Fractal>(
    fractal: &T,
    view: &ComplexView,
    image: &mut Image,
    colorer: &FractalColorType,
) {
    let w_pixels = image.width as usize;
    let buffer = image.get_image_data_mut();
    buffer
        .par_chunks_mut(w_pixels)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, px) in row.iter_mut().enumerate() {
                // convert from screen-space to the mandelbrot space
                let c = view.get_pixel_value(x, y);
                let res = fractal.iterate(c);
                let color = colorer.get_color(&res);
                *px = color.into();
            }
        });
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width();
    let h = screen_height();

    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    let mut view = ComplexView::new(w as usize, h as usize, Dimension(-1.75, 1.75), C64::new());

    let mut fractal = Julia {
        max_iter: 200,
        c: C64(-0.5125, 0.5213),
    };

    let color = FractalColorType::Smooth(100.);

    update_fractal_texture(&fractal, &view, &mut image, &color);

    loop {
        let dt = get_frame_time() as f64;

        // Zoom
        if is_key_down(KeyCode::Z) {
            let factor = 2f64.powf(ZOOM_RATE * dt);
            view.zoom(factor);
        }
        if is_key_down(KeyCode::X) {
            let factor = 2f64.powf(-ZOOM_RATE * dt);
            view.zoom(factor);
        }
        // Pan
        if is_key_down(KeyCode::Up) {
            view.scaled_offset(&C64(0., PAN_RATE * dt));
        }
        if is_key_down(KeyCode::Down) {
            view.scaled_offset(&C64(0., -PAN_RATE * dt));
        }
        if is_key_down(KeyCode::Left) {
            view.scaled_offset(&C64(-PAN_RATE * dt, 0.));
        }
        if is_key_down(KeyCode::Right) {
            view.scaled_offset(&C64(PAN_RATE * dt, 0.));
        }

        if is_key_down(KeyCode::Space) {
            view.reset();
        }

        if is_any_key_down() {
            update_fractal_texture(&fractal, &view, &mut image, &color);
        }

        if is_key_pressed(KeyCode::P) {
            let mouse_pos = C64(
                mouse_position().0 as f64 / w as f64,
                mouse_position().1 as f64 / h as f64,
            );
            fractal.c = mouse_pos;
        }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);
        draw_fps();
        next_frame().await;
    }
}
