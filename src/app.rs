use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use Grid;
use Title;


pub enum State {
    Game(Grid),
    Title(Title)
}

pub struct App {
    gl : GlGraphics,
    state: State 
}


impl App {

    pub fn render (&mut self, glyph: &mut GlyphCache, args: &RenderArgs) {
        match &self.state {
            State::Game(grid) => {
                self.gl.draw(args.viewport(), |c, gl|{
                    grid.render(gl, &c.transform, glyph, args);
                });
            },
            State::Title(title) => {
                self.gl.draw(args.viewport(), |c, gl|{
                    title.render(gl, &c.transform, glyph, args);
                });
            }
        }    
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        //nothing yet
        match &mut self.state {
            State::Game(grid) => {grid.update();},
            State::Title(title) => {
                title.update();
            }
        }
    }


    pub fn click(&mut self, raw_x: f32, raw_y: f32, w: u32, h: u32) {
        //let _state = self.state.clone();
        let new_state = match &mut self.state {
            State::Game(grid) => {
                let cell_width = w as f32 / grid.x_cells as f32;
                let cell_height = h as f32 / grid.y_cells as f32;
                let cell_x = (raw_x as f32 / cell_width) as u32;
                let cell_y = (raw_y as f32 / cell_height) as u32;
                grid.click(cell_x, cell_y);
                None
            },
            State::Title(title) => {
                Some(State::Game(
                        Grid::new(title.grid_w,
                                  title.grid_h,
                                  w,
                                  h,
                                  &title.grid_img_path)
                ))
            }
        };

        if let Some(state) = new_state {
            self.state = state;
        }

    }

    pub fn new(width: u32, height: u32, gl: GlGraphics) -> App {
        App {
            gl: gl,
            state: State::Title(Title::new())
                //grid: Grid::new(5,5, width, height, "./res/sample.jpg")
        }
    }
}



