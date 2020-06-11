extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
use glutin_window::{GlutinWindow, OpenGL};
use graphics::character::CharacterCache;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::input::{Button, ButtonState, Key};
use piston::{EventLoop, EventSettings, Events, RenderEvent, WindowSettings};

type Colour = [f32; 4];
type Map = Vec<Vec<Tile>>;

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

#[derive(Clone)]
struct Object {
    x: i32,
    y: i32,
    character: char,
    colour: Colour,
}
impl Object {
    pub fn new(x: i32, y: i32, character: char, colour: Colour) -> Self {
        Object {
            x,
            y,
            character,
            colour,
        };
    }
}
#[derive(Clone)]
struct Tile {
    colour: Colour,
}
impl Tile {
    pub fn empty() -> Self {
        Tile { colour: WHITE }
    }
    pub fn wall() -> Self {
        Tile { colour: BLACK }
    }
}
fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
    map
}
fn main() {
    let map = make_map();
    let mut window: GlutinWindow = WindowSettings::new("Rogue", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let opengl = OpenGL::V3_2;
    let mut gl = GlGraphics::new(opengl);

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/Cascadia-Code.ttf", (), texture_settings)
        .expect("Could not load font");
        
    let mut player: Object = Object::new(0, 0, '@', RED);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.2; 4], g);
                for i in 0..WORLD_SIZE {
                    for j in 0..WORLD_SIZE {
                        let pos: [f64; 4] = [
                            PIXEL_SIZE * i as f64,
                            PIXEL_SIZE * j as f64,
                            PIXEL_SIZE * (i + 1) as f64,
                            PIXEL_SIZE * (j + 1) as f64,
                        ];

                        graphics::Rectangle::new(map[i as usize][j as usize].colour).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }

                use graphics::Transformed;
                let character = glyphs.character(PIXEL_SIZE, player.character).unwrap();
               
            });
        };
    }
}
