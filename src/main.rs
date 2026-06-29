mod complex;
mod fractal;
mod renderer;
mod view;

use macroquad::prelude::*;

use crate::{complex::C64, fractal::FractalType, renderer::Renderer, view::ComplexView};

const ZOOM_RATE: f64 = 2.; // # of doubles per second
const PAN_RATE: f64 = 2.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fractal Renderer".to_owned(),
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
    let mut view = ComplexView::new(vec2(w, h), vec2(w, h), C64(-2., 2.), C64(0., 0.));

    // Shader
    let mut fractal_type = FractalType::Mandelbrot;

    // Renderer
    let mut renderer = Renderer::new(fractal_type.make());

    // Toggle rendering the orbit of values under the mouse pointer
    let mut render_orbits = false;

    loop {
        let dt = get_frame_time() as f64;
        let mouse_pos: Vec2 = mouse_position().into();

        // Switch fractal. Must run BEFORE render()
        if is_key_pressed(KeyCode::R) {
            fractal_type = fractal_type.next();
            renderer.set_fractal(fractal_type.make());
            view.reset();
        }
        // Toggle rendering orbits
        if is_key_pressed(KeyCode::O) {
            render_orbits = !render_orbits;
        }
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
            view.scaled_offset(C64(0., PAN_RATE * dt));
        }
        if is_key_down(KeyCode::Down) {
            view.scaled_offset(C64(0., -PAN_RATE * dt));
        }
        if is_key_down(KeyCode::Left) {
            view.scaled_offset(C64(-PAN_RATE * dt, 0.));
        }
        if is_key_down(KeyCode::Right) {
            view.scaled_offset(C64(PAN_RATE * dt, 0.));
        }

        if is_key_pressed(KeyCode::Space) {
            view.reset();
        }

        renderer.render(&view);

        // render orbits
        if render_orbits && let Some(c) = view.screen_to_complex(&mouse_pos) {
            // map points in the orbit to the screen
            let points: Vec<Vec2> = renderer
                .fractal()
                .orbit(c)
                .iter()
                .map(|c| view.complex_to_screen(c))
                .collect();
            for p in points.windows(2) {
                if let Some(l) = view.clip_line((p[0], p[1])) {
                    draw_line(l.0.x, l.0.y, l.1.x, l.1.y, 2., GREEN);
                }
            }
        }

        draw_fps();
        next_frame().await;
    }
}
