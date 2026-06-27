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
    let mut view = ComplexView::new(vec2(0., 0.), vec2(w, h), C64(-2., 2.), C64(0., 0.));

    // Shader
    let mut fractal_type = FractalType::Julia;

    // Renderer
    let mut renderer = Renderer::new(fractal_type.make());

    loop {
        let dt = get_frame_time() as f64;

        // Switch fractal. Must run BEFORE render(): set_fractal drops the old
        // Material (which deletes its GPU pipeline), but macroquad defers draw
        // calls until next_frame(). If a pending draw call still references the
        // dropped pipeline, the flush panics in quad_gl. Handling input first
        // means the previous frame's draw calls were already flushed.
        if is_key_pressed(KeyCode::R) {
            fractal_type = fractal_type.next();
            renderer.set_fractal(fractal_type.make());
            view.reset();
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

        if is_key_down(KeyCode::Space) {
            view.reset();
        }

        renderer.render(&view);
        draw_fps();
        next_frame().await;
    }
}
