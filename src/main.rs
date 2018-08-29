extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod app;
mod grid;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use app::App;
use grid::Grid;


fn main() {

    let opengl = OpenGL::V3_2;
    let (mut window_width,mut window_height) = (400,400);

    let mut window: Window = WindowSettings::new(
        "Slide Puzzle",
        [window_width,window_height]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(window_width,
                           window_height,
                           GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());

    let (mut mx,mut my) = (0.0,0.0);

    let mut glyph = GlyphCache::new("res/FiraSans-Regular.ttf", (),
                                opengl_graphics::TextureSettings::new())
                            .unwrap();

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&mut glyph, &r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        e.mouse_cursor(|x,y| {
            mx = x;
            my = y;
        });
        e.resize(|w, h|{
            window_width = w;
            window_height = h;
        });

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            println!("Clicked {},{}", mx, my);
            app.click(mx as f32,
                      my as f32,
                      window_width, 
                      window_height);

        }
    }
}
