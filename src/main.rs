extern crate piston_window;
use piston_window::*;

mod phaser;

fn main() {
    let mut window: PistonWindow = PistonWindow::new(
        OpenGL::V3_3,
        0,
        WindowSettings::new("TJA Sim", [640, 480])
            .opengl(OpenGL::V3_3)
            .srgb(false)
            .exit_on_esc(true)
            .build()
            .unwrap(),
    );
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 640.0, 480.0],
                c.transform,
                g,
            );
        });
    }
}
