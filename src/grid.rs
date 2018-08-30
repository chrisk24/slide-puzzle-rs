
extern crate opengl_graphics;
extern crate rand;
extern crate image;

use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;


pub enum GameEvent {
    Completed,
    NoEvent
}


pub struct Cell {
    pub x_pos: u32, 
    pub y_pos: u32,
    pub content:Option<u32>
}


impl Cell {
    //functions for cell
    pub fn render(&self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  x_cells: u32,
                  y_cells: u32,
                  glyph: &mut GlyphCache,
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

                let text_content = match &self.content {
                    Some(ind) => format!("{}", ind+1),
                    None => "Error".to_string()
                };

                let text_color: [f32; 4] = [0.0,0.0,0.0,1.0];

                text::Text::new_color(text_color, 24).draw(&text_content,
                                                           glyph,
                                                           &DrawState::default(),
                                                           transform.trans(
                                                               5.0, 25.0
                                                           ),
                                                           gl).unwrap();
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
    
    pub fn correct_spot(&self, expect_content: Option<u32>) -> bool { 
        self.content == expect_content
    }

    pub fn click(&mut self) {
        //println!("Clicked Cell ({}, {})", self.x_pos, self.y_pos);
    }
}


pub struct Grid {
    pub x_cells: u32,
    pub y_cells: u32,
    empty_x: u32,
    empty_y: u32,
    img_tiles: Vec<opengl_graphics::Texture>,
    cells: Vec<Cell>,
    pub move_counter: u32
}

impl Grid {
    //functions for grid
    pub fn render(&self, 
                  gl: &mut GlGraphics, 
                  t: &math::Matrix2d,
                  glyph: &mut GlyphCache,
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
                        glyph,
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


    fn is_adjacent(a: (u32, u32), b: (u32, u32)) -> bool {
        let (ax, ay) = a;
        let (bx, by) = b;

        ((ax as i32 - bx as i32).abs() == 1 &&
         (ay == by)) ||
            ((ay as i32 - by as i32).abs() == 1 &&
             (ax == bx))
    }


    fn swap_cells(&mut self, a: (u32, u32), b: (u32, u32)) {
        let (ax, ay) = a;
        let (bx, by) = b;

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


    pub fn click(&mut self, x_cell: u32, y_cell: u32) -> GameEvent {
        let index = Grid::get_grid_index(x_cell,y_cell,self.x_cells);
        if let Some(cell) = self.cells.get_mut(index as usize) {
            cell.click();
        }

        let (move_to_x, move_to_y) = (self.empty_x, self.empty_y);

        //real code
        if Grid::is_adjacent((x_cell, y_cell),
        (move_to_x, move_to_y)) {
            println!("Clicked next to empty!");

            self.swap_cells((x_cell, y_cell),
            (move_to_x, move_to_y));

            self.empty_x = x_cell;
            self.empty_y = y_cell;

            self.move_counter += 1;
        }

        if self.empty_x == self.x_cells - 1 &&
           self.empty_y == self.y_cells - 1 &&
           self.is_solved() {
            return GameEvent::Completed;
        }

        GameEvent::NoEvent
    }

    pub fn mouse_move(&mut self, raw_x: u32, raw_y: u32, w: u32, h: u32) {
        
    }

    pub fn is_solved(&self) -> bool {
        for cell in &self.cells {
            let (cx, cy) = (
                    cell.x_pos,
                    cell.y_pos
                );
            let expect = if cx == self.x_cells - 1 &&
                            cy == self.y_cells - 1 {
                            None
                        } else {
                            Some(Grid::get_grid_index(cx,cy,self.x_cells))
                        };
            if !cell.correct_spot(expect) {
                return false;
            }   
        }

        true
    }

    pub fn get_tile(&self, index: u32) -> &opengl_graphics::Texture {
        self.img_tiles.get(index as usize).unwrap()
    }


    //generate all neighbouring states, 
    //and go to one of the neighbouring states
    //at random
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
        //println!("Moving:{:?}", &rand_move);

        if let Some(mv) = rand_move {
            self.swap_cells(*mv, (ex, ey));
            let (mx, my) = *mv;
            self.empty_x = mx;
            self.empty_y = my;
        }
    }

    //begin at end state, do a random graph transversal
    //with the specified depth
    pub fn randomize(&mut self, depth: u32) {
        //cells, empty_x, empty_y
        //swap_cells
        println!("Randomizing...");
        let mut rng = rand::thread_rng();
        for i in 0..depth {
            self.random_step(&mut rng);
        }
        println!("Randomized...");
    }

    pub fn new (x_cells: u32, 
                y_cells: u32, 
                width: u32, 
                height: u32, 
                img_path: &str) -> Grid {


        //prep the image
        println!("Loading Image....");
        let base_img = image::open(img_path).unwrap()
            .resize_exact(width,height,image::FilterType::Triangle);


        println!("Creating Tiles...");
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
        println!("Tiles Created...");

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
            cells: cells,
            move_counter: 0
        };

        grid.randomize(x_cells * y_cells * 10);
        grid
    }
}


