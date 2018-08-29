extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;

pub struct Button {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub label: String
}

impl Button {
    pub fn render(&self,
                  gl: &mut GlGraphics,
                  t: &math::Matrix2d,
                  glyph: &mut GlyphCache,
                  args: &RenderArgs
                  ) {
        let rect = rectangle::square(0.0,0.0,1.0);
        rectangle([0.2,0.2,0.2,0.8],
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
        let bg_col: [f32; 4] = [0.2,0.5,0.5,1.0];
        clear(bg_col, gl);

        self.play_btn.render(gl,t,glyph,args);

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

    pub fn update(&mut self) {

    }

    pub fn click(&mut self, raw_x: u32, raw_y: u32, w: u32, h: u32) -> TitleEvent {
        if self.play_btn.in_bound(raw_x, raw_y) {
            return TitleEvent::PlayClick;
        } 
        
        TitleEvent::NoEvent
    }

    pub fn new() -> Self {
        Title {
            grid_w: 5,
            grid_h: 5,
            grid_img_path: "./res/sample.jpg".to_string(),
            play_btn: Button {
                x: 5,
                y: 75,
                w: 100,
                h: 50,
                label: "-> Play".to_string()
            }
        }
    }
}
