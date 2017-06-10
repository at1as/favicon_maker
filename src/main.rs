extern crate image;
extern crate imageproc;
extern crate rusttype;

use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use image::{Rgb, RgbImage};
use rusttype::Scale;
use std::env;
use std::path::Path;


pub fn main() {

  /* 
     Usage: 
      cargo run {favicon text} {horizontal offset as % (positive number)}
      cargo run FM 30
   */
  let icon_text: String = match env::args().nth(1) {
    Some(icon_text) => icon_text.to_string(),
    None => "FM".to_string() /* Default FaviconMaker "FM" */
  };

  create_favicon(&icon_text, "./favicon.ico",    16u32);
  create_favicon(&icon_text, "./favicon32.png",  32u32);
  create_favicon(&icon_text, "./favicon48.png",  48u32); 
  create_favicon(&icon_text, "./favicon64.png",  64u32); 
  create_favicon(&icon_text, "./favicon512.png", 512u32); 
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

  /* Font Parameters */
  let font = include_bytes!("assets/DejaVuSans.ttf") as &[u8];
  let height = dimensions; 
  let scale = Scale {
    x: (height as f32) * 0.8,
    y: (height as f32) * 0.8
  };

  let horizontal_offset: u32 = match env::args().nth(2) {
    Some(h_offset) => {
      let percent_offset = h_offset.parse::<f32>().unwrap_or(0.0) / 100 as f32;
      (percent_offset * dimensions as f32) as u32
    },
    _ =>
      0
  };
  let vertical_offset = dimensions as f32 * 0.1;


  draw_text_mut(&mut image, white, horizontal_offset, vertical_offset as u32, scale, font, &txt.trim());

  let _ = image.save(path).unwrap();
}
