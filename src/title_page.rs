extern crate opengl_graphics;
//extern crate tinyfiledialogs;

use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use graphics::character::CharacterCache;
use tinyfiledialogs;


pub enum ButtonPos {
    Fixed((u32, u32)), //fixed with xpos, ypos
    Centered(u32), //centered with ypos
    CenteredOffset((i32, u32)) //xoffset (can be negative), ypos
}

pub enum ButtonState {
    Normal,
    Hover
}

pub struct Button {
    pub pos: ButtonPos,
    pub w: u32,
    pub h: u32,
    pub label: String,
    pub state: ButtonState,
}

impl Button {
    fn get_left_x(&self, screen_width: u32) -> u32 {
        let x = (screen_width as i32 - self.w as i32) / 2;
        if x > 0 {
            x as u32
        } else {
            0
        }
    }
    
    fn get_upper_left(&self, screen_width: u32) -> (u32, u32) {
        match self.pos {
            ButtonPos::Fixed((x,y)) => (x,y),
            ButtonPos::Centered(y) => (self.get_left_x(screen_width), y),
            ButtonPos::CenteredOffset((xoff, y)) => {
                let xpos = self.get_left_x(screen_width) as i32 + xoff;
                let xpos = if xpos > 0 {
                                xpos as u32
                            } else {
                                0 
                            };
                (xpos, y)
            }
        }
    }

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

        let screen_width = args.width as u32;

        let (xpos, ypos) = self.get_upper_left(screen_width);
        let base_pos_transform = t.trans(xpos as f64, ypos as f64);


        rectangle(color,
                  rect, 
                  base_pos_transform.scale(self.w as f64, 
                                           self.h as f64),
                  gl);

        let font_size = 18;

        let text_width = match glyph.width(font_size, &self.label) {
            Ok(x) => x,
            Err(e) => 0.0
        };
        //println!("{}", text_width);

        text::Text::new_color([1.0,1.0,1.0,1.0], font_size)
            .draw(&self.label, glyph, &DrawState::default(),
            base_pos_transform.trans((self.w as f64 - text_width)/2.0, 
                                     25.0), 
            gl)
            .unwrap();
    }

    pub fn in_bound(&self, x: u32, y: u32, w: u32, h: u32) -> bool {

        let (xbound, ybound) = self.get_upper_left(w);
         
        (x >= xbound && 
         x <= xbound + self.w &&
         y >= ybound &&
         y <= ybound + self.h)
    }

    pub fn mouse_move(&mut self, mx: u32, my: u32, w: u32, h: u32) {
        self.state = if self.in_bound(mx,my, w, h){
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
    pub high_score: u32,
    logo: opengl_graphics::Texture,
    play_btn: Button,
    file_choose_btn: Button,
    width_btn: Button,
    height_btn: Button
}

impl Title {

    fn render_text(s: &str, 
                   glyph: &mut GlyphCache, 
                   t: math::Matrix2d,
                   font_size: u32,
                   gl: &mut GlGraphics){
        text::Text::new_color([0.0,0.0,0.0,1.0], font_size)
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
        let bg_col: [f32; 4] = [1.0,1.0,1.0,1.0];
        clear(bg_col, gl);

        self.play_btn.render(gl,t,glyph,args);
        self.file_choose_btn.render(gl, t, glyph, args);
        self.width_btn.render(gl,t,glyph,args);
        self.height_btn.render(gl,t,glyph,args);

        let (screen_width, screen_height) = (args.height as f64,
                                             args.width as f64);
        
        //render the logo
        let logo_scale: f64 = 0.7;
        let logo_width: f64 = 150.0;
        image(&self.logo, t.trans((screen_width - logo_width * logo_scale)/2.0,
                                  25.0)
                            .scale(logo_scale,logo_scale), 
              gl); 


        let text_content = &format!("Path:{}", &self.grid_img_path);

        Title::render_text(text_content,
                           glyph,
                           t.trans(5.0, screen_height - 25.0),
                           24,
                           gl);

        let text_content = &format!("W:{}, H:{}", self.grid_w, self.grid_h);
        Title::render_text(text_content,
                           glyph,
                           t.trans(5.0, screen_height - 50.0),
                           24,
                           gl);

        let text_content = &format!("High Score: {}", self.high_score);
        Title::render_text(text_content,
                           glyph,
                           t.trans(5.0, screen_height - 75.0),
                           24,
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
        if self.play_btn.in_bound(raw_x, raw_y, w, h) {
            return TitleEvent::PlayClick;
        }

        if self.file_choose_btn.in_bound(raw_x, raw_y, w, h) {
            let fl = Self::choose_file(Some("./res/sample.jpg".to_string()));
            println!("{:?}", fl);
            if let Some(pth) = fl {
                self.grid_img_path = pth;
            }
        }

        if self.width_btn.in_bound(raw_x, raw_y, w, h) {
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

        if self.height_btn.in_bound(raw_x, raw_y, w, h) {
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

    pub fn mouse_move(&mut self, raw_x: u32, raw_y: u32, w: u32, h: u32) {
       self.play_btn.mouse_move(raw_x, raw_y, w, h);
       self.file_choose_btn.mouse_move(raw_x, raw_y, w, h);
       self.width_btn.mouse_move(raw_x, raw_y, w, h);
       self.height_btn.mouse_move(raw_x, raw_y, w, h);
    }

    pub fn new(hs : u32) -> Self {
        Title {
            grid_w: 5,
            grid_h: 5,
            grid_img_path: "./res/sample.jpg".to_string(),
            high_score: hs,
            logo: opengl_graphics::Texture::from_path(
                "./res/logo.png",
                &opengl_graphics::TextureSettings::new()
                ).unwrap(),
            play_btn: Button {
                pos: ButtonPos::Centered(145),
                w: 140,
                h: 40,
                label: "Play Game!".to_string(),
                state: ButtonState::Normal
            },
            file_choose_btn: Button {
                pos: ButtonPos::Centered(195),
                w: 160,
                h: 40,
                label: "Choose Image".to_string(),
                state: ButtonState::Normal
            },
            width_btn: Button {
                pos: ButtonPos::CenteredOffset((-25, 245)),
                w: 40,
                h: 40,
                label: "W".to_string(),
                state: ButtonState::Normal
            },
            height_btn: Button {
                pos: ButtonPos::CenteredOffset((25,245)),
                w: 40,
                h: 40,
                label: "H".to_string(),
                state: ButtonState::Normal
            }
        }
    }
}
