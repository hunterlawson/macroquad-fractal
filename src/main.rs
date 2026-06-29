mod complex;
mod fractal;
mod renderer;
mod view;

use std::env;

use macroquad::prelude::*;

use crate::{complex::C64, fractal::FractalType, renderer::Renderer, view::ComplexView};

const ZOOM_RATE: f64 = 2.; // # of doubles per second
const PAN_RATE: f64 = 2.;
const DEFAULT_MAX_ITER: u32 = 200;
const ITER_DELTA: u32 = 10;

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
    let mut view = ComplexView::new(vec2(0., 0.), vec2(w, h), C64(-2., 2.), C64(0., 0.));

    // Shader
    let mut fractal_type = FractalType::Mandelbrot;

    // Renderer
    let mut renderer = Renderer::new(fractal_type.make());

    // Toggle fractal input values under mouse pointer
    let mut fractal_input = false;
    let mut last_mouse_pos: Vec2 = mouse_position().into();

    // Toggle rendering the orbit of values under the mouse pointer
    let mut render_orbits = false;

    // Toggle overlay
    let mut render_overlay = true;

    loop {
        let dt = get_frame_time() as f64;
        let mouse_pos: Vec2 = mouse_position().into();
        let (_, mouse_wheel_y) = mouse_wheel();

        // Switch fractal. Must run BEFORE render()
        if is_key_pressed(KeyCode::R) {
            fractal_type = fractal_type.next();
            renderer.set_fractal(fractal_type.make());
            view.reset();
        }
        // Toggle fractal input values under mouse pointer
        if is_key_pressed(KeyCode::I) {
            fractal_input = !fractal_input;
        }
        // Toggle rendering orbits
        if is_key_pressed(KeyCode::O) {
            render_orbits = !render_orbits;
        }
        // Toggle rendering overlay
        if is_key_pressed(KeyCode::T) {
            render_overlay = !render_overlay;
        }
        // Zoom with keys
        if is_key_down(KeyCode::Z) {
            let factor = 2f64.powf(ZOOM_RATE * dt);
            view.zoom(factor);
        }
        if is_key_down(KeyCode::X) {
            let factor = 2f64.powf(-ZOOM_RATE * dt);
            view.zoom(factor);
        }
        // Zoom with mouse scroll wheel
        if mouse_wheel_y != 0. {
            let factor = 2f64.powf(mouse_wheel_y as f64 * dt / 1.5);
            view.zoom(factor);
        }
        // Iter change
        if is_key_pressed(KeyCode::Up) {
            renderer.set_fractal_max_iter(renderer.fractal_max_iter() + ITER_DELTA);
        }
        if is_key_pressed(KeyCode::Down) && renderer.fractal_max_iter() >= ITER_DELTA {
            renderer.set_fractal_max_iter(renderer.fractal_max_iter() - ITER_DELTA);
        }
        // Pan
        if is_key_down(KeyCode::W) {
            view.scaled_offset(C64(0., PAN_RATE * dt));
        }
        if is_key_down(KeyCode::S) {
            view.scaled_offset(C64(0., -PAN_RATE * dt));
        }
        if is_key_down(KeyCode::A) {
            view.scaled_offset(C64(-PAN_RATE * dt, 0.));
        }
        if is_key_down(KeyCode::D) {
            view.scaled_offset(C64(PAN_RATE * dt, 0.));
        }
        // Reset view
        if is_key_pressed(KeyCode::Space) {
            view.reset();
        }

        // Input into fractal if enabled
        if fractal_input
            && let Some(point) = view.screen_to_complex(&mouse_pos)
            && mouse_pos != last_mouse_pos
        {
            renderer.set_fractal_input_parameter(point);
        }

        // Re-render if not cached
        let cached_render = !renderer.render_cached(&view);

        renderer.draw();

        // Render orbits
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

        // Draw display elements
        if render_overlay {
            let text_elements = vec![
                format!("FPS: {}", get_fps()),
                format!("Fractal: {}", fractal_type),
                format!("Max iterations: {}", renderer.fractal_max_iter()),
                format!("View: {}", view),
                format!(
                    "C: {}",
                    view.screen_to_complex(&mouse_pos).unwrap_or(C64::new())
                ),
                format!("Input C: {}", fractal_input),
                format!("View orbits: {}", render_orbits),
                format!("Cached render: {}", cached_render)
            ];
            for (i, element) in text_elements.iter().enumerate() {
                draw_text(element, 0., 20. + i as f32 * 20., 25., WHITE);
            }
        }

        // Save screenshot
        if is_key_pressed(KeyCode::Q) {
            let image = get_screen_data();
            let file_path = rfd::FileDialog::new()
                .set_title("Save Screenshot As")
                .set_file_name("screenshot.png")
                .set_directory(env::current_dir().unwrap())
                .save_file();

            if let Some(path) = file_path {
                image.export_png(path.to_str().unwrap());
            }
        }

        last_mouse_pos = mouse_pos;

        next_frame().await;
    }
}
