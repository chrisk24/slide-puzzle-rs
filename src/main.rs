extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::*;

pub struct Grid {
    x_cells: i32,
    y_cells: i32,
    cells: Vec<Cell>
}

impl Grid {
    //functions for grid
    pub fn render(&mut self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  args: &RenderArgs)  {
        

        clear([1.0,0.0,0.0,1.0], gl);
        //self.cells.iter().map(|c| c.render(gl));


        for cell in &mut self.cells {
            cell.render(gl,
                     t,
                     self.x_cells,
                     self.y_cells,
                     args);
        }
    }

    pub fn update(&mut self) {
        //self.cells.iter().map(|c| c.update());

        for c in &mut self.cells {
            c.update();
        }
    }

    pub fn new (x_cells: i32, y_cells: i32) -> Grid {
        let mut cells: Vec<Cell> = Vec::new();
        for x in 0..x_cells {
            for y in 0..y_cells {
                let col = (x+y) as f32 /
                          (x_cells + y_cells) as f32;
                cells.push(
                        Cell::new(
                            x,
                            y,
                            Some(col) 
                            )
                    );
            }
        }
        
        
        Grid {
            x_cells: x_cells,
            y_cells: y_cells,
            cells: cells
        }
    }
}

pub struct Cell {
    x_pos: i32, 
    y_pos: i32,
    content:Option<f32>
}


impl Cell {
    //functions for cell
    pub fn render(&mut self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  x_cells: i32,
                  y_cells: i32,
                  args: &RenderArgs) {
        let col: [f32; 4] = match self.content {
            Some(val) => [val,val,val, 1.0],
            None => [1.0,1.0,1.0,1.0]
        };

        
        let width = args.width as f32 / x_cells as f32;
        let height = args.height as f32 / y_cells as f32;
        let x_offset = width * self.x_pos as f32;
        let y_offset = height * self.y_pos as f32;

//        println!("x:{}, y:{}, col:{:?}", x_offset, y_offset, col);
        
        let square = rectangle::square(0.0,0.0,width as f64);
        let transform = t.trans(x_offset as f64,
                                y_offset as f64);

        rectangle(col, square, transform, gl);
    }

    pub fn update(&mut self) {

    }

    pub fn new(x: i32, y: i32, val: Option<f32>) -> Cell {
        Cell {
            x_pos: x,
            y_pos: y,
            content: val
        }
    }
}

pub struct App {
    gl : GlGraphics,
    grid: Grid
}


impl App {

    pub fn render (&mut self, args: &RenderArgs) {

        //let t = Context::new_viewport(args.viewport()).transform;
        let grid: &mut Grid = &mut self.grid; 
        self.gl.draw(args.viewport(), |c, gl|{
            grid.render(gl, &c.transform, args);
        });
        


        //self.grid.render(&mut self.gl, &t, args);     
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        //nothing yet
        self.grid.update();
    }

    pub fn new(gl: GlGraphics) -> App {
        App {
            gl: gl,
            grid: Grid::new(5,5)
        }
    }
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Slide Puzzle",
        [400,400]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
