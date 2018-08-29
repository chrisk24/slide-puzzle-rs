use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use Grid;
use Title;
use TitleEvent;
use GameEvent;

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
                let event = grid.click(cell_x, cell_y);
                match event {
                    GameEvent::Completed => {
                        println!("Congratz!");
                        Some(State::Title(Title::new()))
                    }
                    GameEvent::NoEvent => {
                        None
                    }
                }
            },
            State::Title(title) => {
                let event = title.click(raw_x as u32, raw_y as u32, w, h);
                match event {
                    TitleEvent::PlayClick => Some(State::Game(
                            Grid::new(title.grid_w,
                                      title.grid_h,
                                      w,
                                      h,
                                      &title.grid_img_path)
                    )),
                    TitleEvent::NoEvent => None
                }
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



