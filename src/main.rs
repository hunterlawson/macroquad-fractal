mod fractal;
mod renderer;

use macroquad::prelude::*;
use rug::Complex;

use crate::{
    fractal::FractalType,
    renderer::{renderer::Renderer, view::View},
};

const ZOOM_RATE: f64 = 1.5; // # of doubles per second
const PAN_RATE: f64 = 500.; // pixels / second
const DEFAULT_MAX_ITER: u32 = 256;
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
    let mut fractal = FractalType::Mandelbrot;
    let mut renderer = match Renderer::new(fractal.make()) {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let mut draw_overlay = true;
    let mut controls_elements = vec![
        "R: Swap between Mandelbrot and Julia sets",
        "Spacebar: Reset the view",
        "I: Enable/disable inputting 'C' into the fractal",
        "O: Enable/disable drawing the orbit of 'C'",
        "T: Enable/disable the overlay",
        "Q: Save a screenshot",
    ];
    controls_elements.reverse();

    let mut draw_orbits = false;
    let mut fractal_input = false;

    let mut last_mouse_pos: Vec2 = mouse_position().into();

    loop {
        let dt = get_frame_time() as f64;
        let mouse_pos: Vec2 = mouse_position().into();
        let (_, mouse_wheel_y) = mouse_wheel();
        let c = view.screen_to_view(&mouse_pos);

        // Pan
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            view.scaled_offset((0., PAN_RATE * dt));
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            view.scaled_offset((0., -PAN_RATE * dt));
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            view.scaled_offset((-PAN_RATE * dt, 0.));
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            view.scaled_offset((PAN_RATE * dt, 0.));
        }
        // Zoom with mouse scroll wheel
        if mouse_wheel_y != 0. {
            let factor = 2f64.powf(mouse_wheel_y as f64 * dt / ZOOM_RATE);
            view.zoom(factor);
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
        // Reset view
        if is_key_pressed(KeyCode::Space) {
            view.reset();
        }

        // Switch fractal. Must run BEFORE render()
        if is_key_pressed(KeyCode::R) {
            fractal = fractal.next();
            renderer.set_fractal(fractal.make());
            view.reset();
        }
        // Toggle rendering orbits
        if is_key_pressed(KeyCode::O) {
            draw_orbits = !draw_orbits;
        }
        // Toggle rendering overlay
        if is_key_pressed(KeyCode::T) {
            draw_overlay = !draw_overlay;
        }
        // Toggle fractal input values under mouse pointer
        if is_key_pressed(KeyCode::I) {
            fractal_input = !fractal_input;
        }
        // Input into fractal if enabled
        if fractal_input
            && let Some(point) = &c
            && mouse_pos != last_mouse_pos
        {
            renderer.set_fractal_input_parameter(&point);
        }

        // Render and draw the fractal
        let cached_render = !renderer.cached_render(&view);
        renderer.draw();

        // Render orbits
        if draw_orbits {
            renderer.draw_orbit(&view, &mouse_pos);
        }

        // Draw overlay elements
        if draw_overlay {
            // Info
            let overlay_elements = vec![
                format!("FPS: {}", get_fps()),
                format!("Fractal: {}", renderer.fractal().fractal_type()),
                format!("Max iterations: {}", renderer.fractal().max_iter()),
                format!("View: {{{}}}", view),
                format!("Center point: {}", view.c()),
                format!("C: {:?}", &c),
                format!("View orbits: {}", draw_orbits),
                format!("Cached render: {}", cached_render),
            ];
            for (i, element) in overlay_elements.iter().enumerate() {
                draw_text(element, 0., 20. + i as f32 * 20., 25., WHITE);
            }
            // Controls
            for (i, element) in controls_elements.iter().enumerate() {
                draw_text(element, 0., h - 10. - i as f32 * 20., 25., WHITE);
            }
        }

        last_mouse_pos = mouse_pos;

        next_frame().await;
    }
}
