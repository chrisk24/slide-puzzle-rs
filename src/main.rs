extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate rand;

use std::path::Path;
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
    img_tiles: Vec<opengl_graphics::Texture>,
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


        for cell in &self.cells {
            let texture = match cell.content {
                Some(i) => Some(self.get_tile(i)),
                None => None
            };
            cell.render(gl,
                        t,
                        self.x_cells,
                        self.y_cells,
                        args,
                        texture
            );
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

        let a_cell_content = self.cells
            .get(a_index as usize)
            .unwrap()
            .content;

        let b_cell_content = self.cells
            .get(b_index as usize)
            .unwrap()
            .content;

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

    pub fn get_tile(&self, index: u32) -> &opengl_graphics::Texture {
        self.img_tiles.get(index as usize).unwrap()
    }

    pub fn random_step<R: rand::Rng>(&mut self, rng: &mut R) {
        let mut possible_moves: Vec<(u32, u32)> = Vec::new();
        //up, down, left, right

        let (ex, ey) = (self.empty_x, self.empty_y);

        //up
        if ey > 0 {
            let mv = (
                ex,
                ey - 1
            );
            possible_moves.push(mv);
        }

        //down
        if ey < self.y_cells - 1 {
            let mv = (
                ex,
                ey + 1
            );
            possible_moves.push(mv);
        }

        //left
        if ex > 0 {
            let mv = (
                ex - 1,
                ey
            );
            possible_moves.push(mv);
        }

        //right
        if ex < self.x_cells - 1 {
            let mv = (
                ex + 1,
                ey
            );
            possible_moves.push(mv);
        }

        let rand_move = rng.choose(&possible_moves);
        println!("Moving:{:?}", &rand_move);

        if let Some(mv) = rand_move {
            self.swap_cells(*mv, (ex, ey));
            let (mx, my) = *mv;
            self.empty_x = mx;
            self.empty_y = my;
        }
    }

    pub fn randomize(&mut self, depth: u32) {
        //cells, empty_x, empty_y
        //swap_cells
        let mut rng = rand::thread_rng();
        for i in 0..depth {
            self.random_step(&mut rng);
        }
    }

    pub fn new (x_cells: u32, 
                y_cells: u32, 
                width: u32, 
                height: u32, 
                img_path: &str) -> Grid {


        //prep the image
        let base_img = image::open(img_path).unwrap()
            .resize(width,height,image::FilterType::Triangle);


        let mut img_tiles: Vec<opengl_graphics::Texture> = Vec::new();
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..x_cells*y_cells {
            let x = i % x_cells;
            let y = i / x_cells;


            cells.push(Cell {
                x_pos: x,
                y_pos: y,
                content: Some(i)
            });


            let subimg = base_img.clone()
                .crop(x*(width/x_cells),
                y*(height/y_cells),
                (width/x_cells),
                (height/y_cells))
                .to_rgba();

            let image: opengl_graphics::Texture = 
                opengl_graphics::Texture::from_image(
                    &subimg, 
                    &opengl_graphics::TextureSettings::new()
                );

            img_tiles.push(image);
        }


        let (empty_x, empty_y) = (x_cells - 1, y_cells - 1);

        cells.get_mut(
            Grid::get_grid_index(empty_x, 
                                 empty_y, 
                                 x_cells) 
            as usize)
            .unwrap()
            .content = None;

        let mut grid = Grid {
            x_cells: x_cells,
            y_cells: y_cells,
            empty_x: empty_x,
            empty_y: empty_y,
            img_tiles: img_tiles,
            cells: cells
        };

        grid.randomize(250);
        grid
    }
}


pub struct Cell {
    x_pos: u32, 
    y_pos: u32,
    content:Option<u32>
}


impl Cell {
    //functions for cell
    pub fn render(&self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  x_cells: u32,
                  y_cells: u32,
                  args: &RenderArgs,
                  texture: Option<&opengl_graphics::Texture>) {

        let width = args.width as f32 / x_cells as f32;
        let height = args.height as f32 / y_cells as f32;
        let x_offset = width * self.x_pos as f32;
        let y_offset = height * self.y_pos as f32;

        let transform = t.trans(x_offset as f64,
                                y_offset as f64);


        match &texture {
            Some(val) => {
                let img: &opengl_graphics::Texture = val;
                let (scale_x, scale_y) = (
                    width as f64 / img.get_width() as f64,
                    height as f64 / img.get_height() as f64
                );
                image(img, transform.scale(
                        scale_x, 
                        scale_y
                ), gl);

                let text = match &self.content {
                    Some(ind) => format!("{}", ind),
                    None => "Error".to_string()
                };


            },
            None => {
                let square = rectangle::square(0.0,0.0,1.0);
                let col:[f32; 4] = [0.1, 0.1, 0.1, 1.0];
                rectangle(col, square, transform.scale(
                        width as f64,
                        height as f64
                ), gl);
            }
        }
    }

    pub fn update(&mut self) {

    }

    pub fn click(&mut self) {
        println!("Clicked Cell ({}, {})", self.x_pos, self.y_pos);
    }
}

pub struct App {
    gl : GlGraphics,
    grid: Grid
}


impl App {

    pub fn render (&mut self, args: &RenderArgs) {

        let grid: &mut Grid = &mut self.grid; 
        self.gl.draw(args.viewport(), |c, gl|{
            grid.render(gl, &c.transform, args);
        });

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

    pub fn new(width: u32, height: u32, gl: GlGraphics) -> App {
        App {
            gl: gl,
            grid: Grid::new(5,5, width, height, "./res/sample.jpg")
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

    let mut app = App::new(window_width,
                           window_height,
                           GlGraphics::new(opengl));

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
