use image::{DynamicImage, ImageBuffer, ImageReader, Rgba};
use slint::{Image, SharedPixelBuffer};

pub async fn load_image(url: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // let mut image_buf = vec![];
    let bytes = reqwest::get(url)
        .await.unwrap()
        .bytes().await.unwrap()
        .to_vec();
    image::load_from_memory(&bytes).unwrap().to_rgba8()

    // slint::Image::from_rgba8(buffer)
}