mod server_communication;
mod wms_communication;
use image::DynamicImage;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[tokio::main]
async fn main() {
    let coordinates: ((f64, f64), (f64, f64)) = (
        (469246.9611715295,5074765.4764674315),
        (469279.91663375776,5074792.545888164),
    );
    let offset = 10.0;

    let trees = match server_communication::get_tree_data_in_area(coordinates).await {
        Ok(_tree_data) => {
            println!("Succesfully recieved tree data");
            _tree_data.features.iter().for_each(|tree| {
                println!(
                    "Stablo {}: ({}, {})",
                    tree.properties.sifra,
                    tree.geometry.coordinates[0],
                    tree.geometry.coordinates[1]
                )
            });
            _tree_data
        }
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let mut trees_photos: Vec<(i32, DynamicImage)> = Vec::new();
    for tree_feature in trees.features {
        match wms_communication::get_tree_ortophoto(
            &(
                tree_feature.geometry.coordinates[0],
                tree_feature.geometry.coordinates[1],
            ),
            offset,
        )
        .await
        {
            Ok(photo) => trees_photos.push((tree_feature.properties.sifra, photo)),
            _ => {}
        };
    }

    let photo_saving_errors: Vec<Result<(), String>> = trees_photos
        .par_iter()
        .map(|x| wms_communication::save_photo_to_file(&x.1, format!("stablo_{}", x.0)))
        .filter(|x| match x {
            Err(_) => true,
            _ => false,
        })
        .collect();

    photo_saving_errors.iter().for_each(|result| match result {
        Ok(_) => {},
        Err(error) => println!("{}", error)
    })
}
