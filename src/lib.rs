use crate::image::ImageError;
use image;

pub fn convert_to_png(path: String) -> Result<(), ImageError> {
    let img = image::open(&path)?;  
    let format = image::ImageFormat::Png;

    let mid = path.len();
    let path = path.split_at(mid - 4);

    let savename = String::from(path.0.to_string() + "_PNG.png");
    img.save_with_format(savename, format)?;
    Ok(())
}