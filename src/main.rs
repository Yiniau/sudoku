#![deny(missing_docs)]

//! A Sudoku game.

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate rand;
// extern crate glutin_window;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, Filter, TextureSettings};
use sdl2_window::Sdl2Window;
// use glutin_window::GlutinWindow;

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("sudoku", (512, 512))
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: Sdl2Window = settings.build()
        .expect("build GlutinWindow failed");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new(
        "assets/FiraSans-Regular.ttf",
        (),
        texture_settings,
    ).expect("load font asset failed");

    gameboard_controller.gameboard.question_generate();

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e
        );

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
