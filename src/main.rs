#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::path::Path;
use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use image::{Rgb, RgbImage};
use rusttype::Scale;


pub fn main() {
  let icon_text = "FM";
  create_favicon(icon_text, "./favicon.ico",   16u32);
  create_favicon(icon_text, "./favicon32.png", 32u32);
  create_favicon(icon_text, "./favicon48.png", 48u32); 
  create_favicon(icon_text, "./favicon64.png", 64u32); 
}

pub fn create_favicon(txt: &str, filepath: &str, dimensions: u32) {

  let white = Rgb([255u8, 255u8, 255u8]);
  let black = Rgb([0u8, 0u8, 0u8]);

  let font_offset = 1u32;
  let path = Path::new(filepath);
  let mut image = RgbImage::new(dimensions, dimensions);

  draw_filled_rect_mut(&mut image,
                       Rect::at(font_offset as i32, font_offset as i32).of_size(dimensions - font_offset, dimensions - font_offset),
                       black);

  let font = include_bytes!("assets/DejaVuSans.ttf") as &[u8];
  let height = dimensions; 
  let scale = Scale { x: (height as f32) * 0.8, y: (height as f32) * 0.8 };

  let vertical_offset = dimensions as f32 * 0.1;

  draw_text_mut(&mut image, white, 0, vertical_offset as u32, scale, font, &txt.trim());

  let _ = image.save(path).unwrap();
}
