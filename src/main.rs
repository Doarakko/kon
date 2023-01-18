use image::load_from_memory;
use std::collections::HashMap;
use viuer::{print, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://randomfox.ca/floof/")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let img_url = &resp["image"];

    let img_bytes = reqwest::get(img_url).await?.bytes().await?;
    let img = load_from_memory(&img_bytes)?;

    let conf = Config {
        y: img.height() as i16,
        ..Default::default()
    };
    print(&img, &conf)?;

    println!("{}", img_url);

    Ok(())
}
