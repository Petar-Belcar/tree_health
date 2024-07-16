use reqwest::get;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
pub struct Geometry {
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
    pub coordinates: [f64; 2]
}

#[derive(Deserialize)]
pub struct Properties {
    pub sifra: i32,
    pub file: String
}

#[derive(Deserialize)]
pub struct Feature {
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
    pub geometry: Box<Geometry>,
    pub properties: Properties
}

#[derive(Deserialize)]
pub struct TreeData {
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
    pub features: Vec<Feature>
}

pub async fn get_tree_data_in_area(coordinates: ((f64, f64), (f64, f64))) -> Result<TreeData, String> {
    let url = format!("https://gis.zrinjevac.hr/stabla_geom.php?bbox={},{},{},{}&srid=3765", coordinates.0.0, coordinates.0.1, coordinates.1.0, coordinates.1.1);

    let res = match get(url).await{
        Ok(res) => match res.text().await {
            Ok(body) => body,
            Err(error) => return Err(format!("Error while getting the body of the response: {}", error.to_string()))
        },
        Err(error) => return Err(format!("Error while sending request to server containing tree data: {}", error.to_string()))
    };

    
    match from_str(&res) {
        Ok(tree_data) => Ok(tree_data),
        Err(error) => Err(format!("Error while deserializing response from server: {}", error))
    }
}
