extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);
    while let Some(e) = events.next(&mut window) {
        
    };

    
}
