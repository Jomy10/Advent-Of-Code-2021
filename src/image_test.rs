use image::{ImageBuffer, RgbImage};

const WIDTH:u32 = 10;
const HEIGHT:u32 = 10;

fn main() {
    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    *image.get_pixel_mut(5, 5) = image::Rgb([255,255,255]);
    image.save("output.png").unwrap();
}
