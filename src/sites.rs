use serde_derive::Deserialize;
use serde_json;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub is_nsfw: Option<bool>,
}

pub fn load_json() -> Result<Vec<Site>, Box<dyn std::error::Error>> {
    let file = File::open("src/assets/sites.json")?; 
    let reader = BufReader::new(file);  
    let sites: Vec<Site> = serde_json::from_reader(reader)?; 
    Ok(sites)  
}
