use image::{self, ImageBuffer, Pixel, Rgba};
use std::path::Path;

const DEF_IMAGE_SIZE: u32 = 256;

pub struct Assets {
    pub tex: Vec<u32>,
    pub width: i32,
    pub height: i32,
}

impl Assets {
    pub fn load() -> Assets {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(DEF_IMAGE_SIZE, DEF_IMAGE_SIZE);

        for pixel in img.enumerate_pixels_mut() {
            *pixel.2 = image::Rgba([255, 255, 255, 255]);
        }
        let image = image::open(Path::new("resources/texture.png"));
        let tex0: ImageBuffer<Rgba<u8>, Vec<u8>> = match image {
            Ok(image_result) => image_result.to_rgba8(),
            Err(_image_error) => img,
        };

        let dims = tex0.dimensions();
        let mut tex = Vec::new();

        for j in 0..dims.1 {
            for i in 0..dims.0 {
                let p = image::ImageBuffer::get_pixel(&tex0, i, j).to_rgba();
                tex.push(
                    256u32.pow(3) * (p[3] as u32)
                        + 256u32.pow(2) * (p[0] as u32)
                        + 256u32.pow(1) * (p[1] as u32)
                        + (p[2] as u32),
                );
            }
        }

        Assets {
            tex,
            width: dims.0 as i32,
            height: dims.1 as i32,
        }
    }
}
