use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde_json;
use std::string::String;


pub struct Config {
    path: String,
    data: serde_json::Value,
}


impl Config {
    pub fn create() -> Config {
        let json = json!({
            "map":{
                "radius":0
            },
            "network": {
                "port": 4451
            }
        });
        let mut config = Config {
            path: "config.json".to_string(),
            data: json
        };

        if config.has_file() {
            config.load();
        } else {
            config.make();
        }
        config
    }

    //Checks if there is config file
    fn has_file(&self) -> bool {
        let exists = Path::new(&self.path).exists();
        exists
    }

    //Loads data from config file
    fn load(&mut self) {
        let mut file = File::open(&self.path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).ok();
        self.data = serde_json::from_str(data.as_str()).unwrap();
    }

    //Creates default config file
    fn make(&self) {
        trace!("Creating default config file");
        let mut file = (File::create(&self.path)).unwrap();
        file.write((serde_json::to_string_pretty(&self.data).unwrap()).as_bytes()).ok();
    }

    pub fn get_map_radius(&self) -> i32 {
        let radius: i32 = self.data["map"]["radius"].to_string().parse().unwrap();
        radius
    }
    
    pub fn get_network_port(&self) -> i32 {
        let radius: i32 = self.data["network"]["port"].to_string().parse().unwrap();
        radius
    }
}