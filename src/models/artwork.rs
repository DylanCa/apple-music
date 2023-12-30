use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Artwork {
    pub data: Option<String>,
    pub description: Option<String>,
    pub downloaded: bool,
    pub format: Option<String>,
    pub kind: i32,
    pub raw_data: String,
}