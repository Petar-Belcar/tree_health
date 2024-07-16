use std::io::Cursor;

use image::{load, DynamicImage, ImageFormat};
use reqwest::get;

pub async fn get_tree_ortophoto(
    tree_coordinates: &(f64, f64),
    offset: f64,
) -> Result<DynamicImage, String> {
    // Offset should be such that the capture images contains the whole tree but not much more than
    // that
    let url = format!("https://geoportal.zagreb.hr/Public/Ortofoto2018_Public/MapServer/WMSServer?REQUEST=GetMap&SERVICE=WMS&VERSION=1.3&FORMAT=image%2Fpng&STYLES=&TRANSPARENT=true&LAYERS=ZGCDOF2018&CRS=EPSG%3A3765&TILED=true&WIDTH=256&HEIGHT=256&BBOX={},{},{},{}", tree_coordinates.0 - offset, tree_coordinates.1 - offset, tree_coordinates.0 + offset, tree_coordinates.1 + offset);

    let res = match get(url).await {
        Ok(photo) => photo,
        Err(error) => {
            return Err(format!(
                "Error occured while requesting an orotophoto: {}",
                error.to_string()
            ))
        }
    };

    let photo_bytes = match res.bytes().await {
        Ok(bytes) => bytes,
        Err(error) => {
            return Err(format!(
                "Error occured while trying to get bytes from response: {}",
                error.to_string()
            ))
        }
    };

    let photo_cursour = Cursor::new(photo_bytes);
    match load(photo_cursour, ImageFormat::Png) {
        Ok(photo) => Ok(photo),
        Err(error) => Err(format!(
            "Error occured while turning recieved bytes into photo: {}",
            error.to_string()
        )),
    }
}

pub fn save_photo_to_file(img: &DynamicImage, image_name: String) -> Result<(), String> {
    match img.save(format!("tree_ortophotos/{}.png", image_name)) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("Error occured while trying to save image {}.png: {}", image_name, error.to_string()))
    }
}
