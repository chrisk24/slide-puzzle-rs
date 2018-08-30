extern crate opengl_graphics;
//extern crate tinyfiledialogs;

use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use tinyfiledialogs;

pub enum ButtonState {
    Normal,
    Hover
}

pub struct Button {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub label: String,
    pub state: ButtonState,
}

impl Button {
    pub fn render(&self,
                  gl: &mut GlGraphics,
                  t: &math::Matrix2d,
                  glyph: &mut GlyphCache,
                  args: &RenderArgs
    ) {
        let color = match &self.state {
            ButtonState::Normal => [0.2,0.2,0.2,1.0],
            ButtonState::Hover => [0.4,0.4,0.4,1.0]
        };
    
        
        let rect = rectangle::square(0.0,0.0,1.0);
        rectangle(color,
                  rect, 
                  t.trans(self.x as f64, self.y as f64)
                  .scale(self.w as f64, self.h as f64),
                  gl);

        text::Text::new_color([1.0,1.0,1.0,1.0], 32)
            .draw(&self.label, glyph, &DrawState::default(),
            t.trans(self.x as f64 + 5.0, self.y as f64 + 25.0)
            .scale(0.7, 0.7), gl)
            .unwrap();
    }

    pub fn in_bound(&self, x: u32, y: u32) -> bool {
        (x >= self.x && 
         x <= self.x+ self.w &&
         y >= self.y &&
         y <= self.y + self.h)
    }

    pub fn mouse_move(&mut self, mx: u32, my: u32) {
        self.state = if self.in_bound(mx,my){
                        ButtonState::Hover
                      }else {
                        ButtonState::Normal
                      };
        /*match &self.state {
            ButtonState::Hover => {println!("Hovered {}", &self.label);},
            ButtonState::Normal => {}
        }*/
    }
}


pub enum TitleEvent {
    PlayClick,
    NoEvent
}


pub struct Title {
    pub grid_w: u32,
    pub grid_h: u32,
    pub grid_img_path: String,
    play_btn: Button,
    file_choose_btn: Button,
    width_btn: Button,
    height_btn: Button
}

impl Title {

    fn render_text(s: &str, 
                   glyph: &mut GlyphCache, 
                   t: math::Matrix2d,
                   gl: &mut GlGraphics){
        text::Text::new_color([0.0,0.0,0.0,1.0], 32)
            .draw(s,
                  glyph,
                  &DrawState::default(),
                  t, 
                  gl)
            .unwrap();
    }

    pub fn render(&self,
                  gl: &mut GlGraphics,
                  t: &math::Matrix2d,
                  glyph: &mut GlyphCache, 
                  args: &RenderArgs) {
        let bg_col: [f32; 4] = [0.2,0.5,0.2,1.0];
        clear(bg_col, gl);

        self.play_btn.render(gl,t,glyph,args);
        self.file_choose_btn.render(gl, t, glyph, args);
        self.width_btn.render(gl,t,glyph,args);
        self.height_btn.render(gl,t,glyph,args);

        let text_content = &format!("Path:{}", &self.grid_img_path);

        Title::render_text(text_content,
                           glyph,
                           t.trans(5.0, 25.0)
                           .scale(0.7, 0.7),
                           gl);

        let text_content = &format!("W:{}, H:{}", self.grid_w, self.grid_h);
        Title::render_text(text_content,
                           glyph,
                           t.trans(5.0, 50.0)
                           .scale(0.7, 0.7),
                           gl);
    }

    pub fn choose_file(path: Option<String>) -> Option<String> {
        let raw_path = match path {
            Some(fl) => fl,
            None => "./res/sample.jpg".to_string()
        };
        let fl = tinyfiledialogs::open_file_dialog("Open", &raw_path, None);
        println!("{:?}", fl);
        fl
    }

    pub fn input_dialog(msg: &str, default: Option<String>) -> Option<String> {
        let def = match default {
            Some(x) => x,
            None => "".to_string()
        };
        let result = tinyfiledialogs::input_box("Input", msg, &def);
        result
    }

    pub fn update(&mut self) {

    }

    pub fn click(&mut self, raw_x: u32, raw_y: u32, w: u32, h: u32) -> TitleEvent {
        if self.play_btn.in_bound(raw_x, raw_y) {
            return TitleEvent::PlayClick;
        }

        if self.file_choose_btn.in_bound(raw_x, raw_y) {
            let fl = Self::choose_file(Some("./res/sample.jpg".to_string()));
            println!("{:?}", fl);
            if let Some(pth) = fl {
                self.grid_img_path = pth;
            }
        }

        if self.width_btn.in_bound(raw_x, raw_y) {
            let new_width = Self::input_dialog("Enter Width", Some("5".to_string()));
            println!("{:?}", new_width);
            if let Some(mut new_width) = new_width {
                new_width.retain(|c| c.is_numeric());
                match new_width.parse::<u32>() {
                    Ok(w) => {self.grid_w = w;},
                    Err(e) => {println!("{:?}", e);}
                }
            }
        }

        if self.height_btn.in_bound(raw_x, raw_y) {
            let new_height = Self::input_dialog("Enter Height", Some("5".to_string()));
            println!("{:?}", new_height);
            if let Some(mut new_height) = new_height {
                new_height.retain(|c| c.is_numeric());
                match new_height.parse::<u32>() {
                    Ok(h) => {self.grid_h = h;},
                    Err(e) => {println!("{:?}", e);}
                }
            }
        }

        TitleEvent::NoEvent
    }

    pub fn mouse_move(&mut self, raw_x: u32, raw_y: u32) {
       self.play_btn.mouse_move(raw_x, raw_y);
       self.file_choose_btn.mouse_move(raw_x, raw_y);
       self.width_btn.mouse_move(raw_x, raw_y);
       self.height_btn.mouse_move(raw_x, raw_y);
    }

    pub fn new() -> Self {
        Title {
            grid_w: 5,
            grid_h: 5,
            grid_img_path: "./res/sample.jpg".to_string(),
            play_btn: Button {
                x: 5,
                y: 75,
                w: 140,
                h: 40,
                label: "Play Game!".to_string(),
                state: ButtonState::Normal
            },
            file_choose_btn: Button {
                x: 5,
                y: 150,
                w: 140,
                h: 40,
                label: "choose img".to_string(),
                state: ButtonState::Normal
            },
            width_btn: Button {
                x: 5,
                y: 225,
                w: 50,
                h: 50,
                label: "W".to_string(),
                state: ButtonState::Normal
            },
            height_btn: Button {
                x: 75,
                y: 225,
                w: 50,
                h: 50,
                label: "H".to_string(),
                state: ButtonState::Normal
            }
        }
    }
}
