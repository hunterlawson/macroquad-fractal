mod fractal;
use fractal::*;

mod complex;
use complex::*;

mod view;
use view::*;

use macroquad::{color::hsl_to_rgb, prelude::*};
use rayon::prelude::*;

// const MAX_ITERATIONS: u32 = 100;
// const START_ZOOM: f64 = 1.;
// // −0.20710786709396773
// const START_C: (f64, f64) = (-0.7391795056397475, 0.20343230913332802);
// const REGION_X: (f64, f64) = (-2., 1.5);

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

fn get_color(x: f64) -> Color {
    const COLOR_RANGE: f64 = 200.;

    let h = (x % COLOR_RANGE) / COLOR_RANGE;

    hsl_to_rgb(h as f32, 0.8, 0.5)
}

fn update_fractal_texture<T: Fractal>(fractal: &T, view: &ComplexView, image: &mut Image) {
    let w_pixels = image.width as usize;
    let buffer = image.get_image_data_mut();
    buffer
        .par_chunks_mut(w_pixels)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, px) in row.iter_mut().enumerate() {
                // convert from screen-space to the mandelbrot space
                let c = view.get_pixel_value(x, y);
                let iterations = fractal.iterate(c);
                let color = match iterations {
                    None => BLACK,
                    Some(x) => get_color(x),
                };
                *px = color.into();
            }
        });
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width();
    let h = screen_height();
    // let resolution = 5.;

    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    let mut view = ComplexView::new(
        w as usize,
        h as usize,
        Dimension(-2., 2.),
        C64(-0.7391795056397475, 0.20343230913332802),
    );

    let mandelbrot = Mandelbrot { max_iter: 100 };

    update_fractal_texture(&mandelbrot, &view, &mut image);

    loop {
        // draw_text(format!("{} + {}i", c_offset.0, c_offset.1), 0., 50., 32., WHITE);

        // if is_key_pressed(KeyCode::S) {
        //     debug!("{}, {}", c_offset.0, c_offset.1);
        // }

        const ZOOM_RATE: f64 = 3.; // doubles per second
        let dt = get_frame_time() as f64;

        if is_key_down(KeyCode::Z) {
            let factor = 2f64.powf(ZOOM_RATE * dt);
            view.zoom(factor);
            update_fractal_texture(&mandelbrot, &view, &mut image);
        }
        if is_key_down(KeyCode::X) {
            let factor = 2f64.powf(-ZOOM_RATE * dt);
            view.zoom(factor);
            update_fractal_texture(&mandelbrot, &view, &mut image);
        }
        // if is_key_down(KeyCode::Right) {
        //     offset = (offset.0 + (1. / zoom) * get_frame_time() as f64, offset.1)
        // }
        // if is_key_down(KeyCode::Left) {
        //     c_offset = (c_offset.0 - (1. / zoom) * get_frame_time() as f64, c_offset.1)
        // }
        // if is_key_down(KeyCode::Up) {
        //     c_offset = (c_offset.0, c_offset.1 - (1. / zoom) * get_frame_time() as f64)
        // }
        // if is_key_down(KeyCode::Down) {
        //     c_offset = (c_offset.0, c_offset.1 + (1. / zoom) * get_frame_time() as f64)
        // }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);
        draw_fps();
        next_frame().await;
    }
}
