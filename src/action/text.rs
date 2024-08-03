extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::Font;

use std::fs;

pub fn draw(
    image: DynamicImage, // No need for `mut` here as we're converting it
    rgba: (u8, u8, u8, u8),
    font: &str,
    (x, y): (f32, f32),
    scale: f32,
    text: &str,
) -> DynamicImage {
    let color = Rgba([rgba.0, rgba.1, rgba.2, rgba.3]);
    let font = match load_font(&font) {
        Err(e) => {
            eprintln!("{:?}", e);
            return image;
        }
        Ok(f) => f,
    };
    let (w, h) = (image.width() as f32, image.height() as f32);
    
    // Convert image to RGBA8
    let mut rgba8_image: RgbaImage = image.to_rgba8();
    
    imageproc::drawing::draw_text_mut(
        &mut rgba8_image,
        color,
        (w * x) as i32,
        (h * y) as i32,
        rusttype::Scale::uniform(w as f32 * (scale * 0.1)),
        &font,
        text,
    );
    
    DynamicImage::ImageRgba8(rgba8_image)
}

fn load_font(name: &str) -> Result<Font<'static>, ()> {
    let bytes = fs::read(name).map_err(|e| {
        eprintln!("loading {}: {}", name, e);
    })?;

    Font::try_from_vec(bytes).ok_or(())
}