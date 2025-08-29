use image::{ImageBuffer, Rgba};

pub async fn load_image(url: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let bytes = reqwest::get(url)
        .await.unwrap()
        .bytes().await.unwrap()
        .to_vec();
    image::load_from_memory(&bytes).unwrap().to_rgba8()
}