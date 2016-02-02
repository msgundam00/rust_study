extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod object;
mod app;

use app::App;
use piston::window::WindowSettings;
use piston::window::Window;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as GLWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use object::*;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: GLWindow = WindowSettings::new(
        "runner",
        [600, 400]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        tiles: vec![],
        player: Player::new(),
        velocity: 100.0
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        } else if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
