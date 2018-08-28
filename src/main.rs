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
    x_cells: u32,
    y_cells: u32,
    empty_x: u32,
    empty_y: u32,
    cells: Vec<Cell>
}

impl Grid {
    //functions for grid
    pub fn render(&mut self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  args: &RenderArgs)  {
        
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        clear(BLACK, gl);


        for cell in &mut self.cells {
            cell.render(gl,
                     t,
                     self.x_cells,
                     self.y_cells,
                     args);
        }
    }

    pub fn update(&mut self) {

        for c in &mut self.cells {
            c.update();
        }
    }


    fn get_grid_index(x: u32, y: u32, w: u32) -> u32{
        x + y * w
    }


    fn is_adjacent(A: (u32, u32), B: (u32, u32)) -> bool {
        let (ax, ay) = A;
        let (bx, by) = B;

        ((ax as i32 - bx as i32).abs() == 1 &&
         (ay == by)) ||
        ((ay as i32 - by as i32).abs() == 1 &&
         (ax == bx))
    }

    
    fn swap_cells(&mut self, A: (u32, u32), B: (u32, u32)) {
        let (ax, ay) = A;
        let (bx, by) = B;
        
        let a_index = Grid::get_grid_index(ax,ay,self.x_cells);
        let b_index = Grid::get_grid_index(bx,by,self.x_cells);

        let a_cell_content = self.cells.get(a_index as usize).unwrap().content;
        let b_cell_content = self.cells.get(b_index as usize).unwrap().content;

        self.cells.get_mut(b_index as usize).unwrap().content = a_cell_content;
        self.cells.get_mut(a_index as usize).unwrap().content = b_cell_content;
    }
    

    pub fn click(&mut self, x_cell: u32, y_cell: u32) {
        let index = Grid::get_grid_index(x_cell,y_cell,self.x_cells);
        if let Some(cell) = self.cells.get_mut(index as usize) {
            cell.click();
        }

        let (empty_x, empty_y) = (self.empty_x, self.empty_y);

        //real code
        if Grid::is_adjacent((x_cell, y_cell),
                       (empty_x, empty_y)) {
            println!("Clicked next to empty!");

            self.swap_cells((x_cell, y_cell),
                            (empty_x, empty_y));

            self.empty_x = x_cell;
            self.empty_y = y_cell;
        }


    }


    pub fn new (x_cells: u32, y_cells: u32) -> Grid {
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..x_cells*y_cells {
            cells.push(Cell::new(0,0,None));
        }

        for x in 0..x_cells {
            for y in 0..y_cells {
                let col = (x+y) as f32 /
                          (x_cells + y_cells) as f32;
                let index = Grid::get_grid_index(x,y,x_cells);
                let cell = cells.get_mut(index as usize).unwrap();
                cell.x_pos = x;
                cell.y_pos = y;
                cell.content = Some(col);
            }
        } 

        let (empty_x, empty_y) = (x_cells - 1, y_cells - 1);

        cells.get_mut(
                Grid::get_grid_index(empty_x, 
                                     empty_y, 
                                     x_cells) 
                    as usize)
             .unwrap()
             .content = None;
        
        Grid {
            x_cells: x_cells,
            y_cells: y_cells,
            empty_x: empty_x,
            empty_y: empty_y,
            cells: cells
        }
    }
}

pub struct Cell {
    x_pos: u32, 
    y_pos: u32,
    content:Option<f32>
}


impl Cell {
    //functions for cell
    pub fn render(&mut self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  x_cells: u32,
                  y_cells: u32,
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

    pub fn click(&mut self) {
        println!("Clicked Cell ({}, {})", self.x_pos, self.y_pos);
    }

    pub fn new(x: u32, y: u32, val: Option<f32>) -> Cell {
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


    pub fn click(&mut self, raw_x: f32, raw_y: f32, w: u32, h: u32) {
        let cell_width = w as f32 / self.grid.x_cells as f32;
        let cell_height = h as f32 / self.grid.y_cells as f32;
        let cell_x = (raw_x as f32 / cell_width) as u32;
        let cell_y = (raw_y as f32 / cell_height) as u32;
        self.grid.click(cell_x, cell_y);
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
    let (mut window_width,mut window_height) = (400,400);

    let mut window: Window = WindowSettings::new(
        "Slide Puzzle",
        [window_width,window_height]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    
    let (mut mx,mut my) = (0.0,0.0);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
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
