mod complex;
mod fractal;
mod renderer;

use macroquad::prelude::*;
use rug::Complex;

use crate::{fractal::FractalType, renderer::{renderer::Renderer, view::View}};

const ZOOM_RATE: f64 = 1.5; // # of doubles per second
const PAN_RATE: f64 = 500.; // pixels / second
const DEFAULT_MAX_ITER: u32 = 200;
const ITER_DELTA: u32 = 10;
const PRECISION: u32 = 53;

fn window_conf() -> Conf {
    Conf {
        window_title: "Perturbation Fractal Renderer".to_owned(),
        // window_width: 800,
        // window_height: 800,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width();
    let h = screen_height();

    // View
    let start_c = Complex::with_val(PRECISION, (0., 0.));
    let mut view = View::new(vec2(0., 0.), vec2(w, h), start_c, vec2(-2., 2.));

    // Renderer
    let fractal = FractalType::Mandelbrot;
    let renderer = match Renderer::new(fractal.make()) {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return;
        },
    };

    let orbit = renderer.fractal().orbit(Complex::with_val(PRECISION, (-0.5, 0.5)));
    debug!("{:?}", orbit);

    loop {
        let dt = get_frame_time() as f64;
        let (_, mouse_wheel_y) = mouse_wheel();

        // Pan
        if is_key_down(KeyCode::W) {
            view.scaled_offset((0., -PAN_RATE * dt));
        }
        if is_key_down(KeyCode::S) {
            view.scaled_offset((0., PAN_RATE * dt));
        }
        if is_key_down(KeyCode::A) {
            view.scaled_offset((-PAN_RATE * dt, 0.));
        }
        if is_key_down(KeyCode::D) {
            view.scaled_offset((PAN_RATE * dt, 0.));
        }
        // Zoom with mouse scroll wheel
        if mouse_wheel_y != 0. {
            let factor = 2f64.powf(mouse_wheel_y as f64 * dt / 1.5);
            view.zoom(factor);
        }

        // Render the fractal
        renderer.render(&view);

        // Draw overlay elements
        let text_elements = vec![
                format!("FPS: {}", get_fps()),
                format!("Fractal: {}", fractal),
                format!("Max iterations: {}", renderer.fractal().max_iter()),
                format!("View: {}", view),
                format!(
                    "C: {}",
                    view.c()
                ),
                // format!("Input C: {}", fractal_input),
                // format!("View orbits: {}", render_orbits),
                // format!("Cached render: {}", cached_render)
            ];
            for (i, element) in text_elements.iter().enumerate() {
                draw_text(element, 0., 20. + i as f32 * 20., 25., WHITE);
            }

        next_frame().await;
    }
}
