use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Artwork {
    pub class: String, // the class of the item

    pub data: Option<String>,
    pub description: Option<String>,
    pub downloaded: bool,
    pub format: Option<String>,
    pub kind: i32,
    pub raw_data: String,
}