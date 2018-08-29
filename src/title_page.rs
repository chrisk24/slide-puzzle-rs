extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;


pub struct Title {
    pub grid_w: u32,
    pub grid_h: u32,
    pub grid_img_path: String
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
        let bg_col: [f32; 4] = [0.0,0.2,0.2,1.0];
        clear(bg_col, gl);

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

    pub fn click(&mut self, raw_x: u32, raw_y: u32, w: u32, h: u32) {

    }

    pub fn new() -> Self {
        Title {
            grid_w: 5,
            grid_h: 5,
            grid_img_path: "./res/sample.jpg".to_string()
        }
    }
}
