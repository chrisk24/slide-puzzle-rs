use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use Grid;



#[derive(Clone)]
pub enum State {
    Game,
    Title
}

pub struct App {
    gl : GlGraphics,
    grid: Grid,
    state: State 
}


impl App {

    pub fn render (&mut self, glyph: &mut GlyphCache, args: &RenderArgs) {
        match self.state {
            State::Game => {
                let grid: &Grid = &self.grid;
                self.gl.draw(args.viewport(), |c, gl|{
                    grid.render(gl, &c.transform, glyph, args);
                });
            },
            State::Title => {
                self.gl.draw(args.viewport(), |c, gl|{
                    clear([0.0,1.0,0.0,1.0], gl);
                });
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        //nothing yet
        match self.state {
            State::Game => {self.grid.update();},
            State::Title => {
            }
        }
    }


    pub fn click(&mut self, raw_x: f32, raw_y: f32, w: u32, h: u32) {
        let _state = self.state.clone();
        match _state {
            State::Game => {
                let cell_width = w as f32 / self.grid.x_cells as f32;
                let cell_height = h as f32 / self.grid.y_cells as f32;
                let cell_x = (raw_x as f32 / cell_width) as u32;
                let cell_y = (raw_y as f32 / cell_height) as u32;
                self.grid.click(cell_x, cell_y);
            }
            State::Title => {
                self.state = State::Game;
            }
        }
    }

    pub fn new(width: u32, height: u32, gl: GlGraphics) -> App {
        App {
            gl: gl,
            state: State::Title,
            grid: Grid::new(3,3, width, height, "./res/sample.jpg")
        }
    }
}



