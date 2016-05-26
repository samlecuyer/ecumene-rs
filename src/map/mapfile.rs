
extern crate geo;
extern crate yaml_rust;

use map::layer::Layer;
use bbox::Bbox;
use self::yaml_rust::{YamlLoader, Yaml};
use self::yaml_rust::yaml;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub struct Map {
    pub name: String,
    pub srs: String,
    pub extent: Bbox,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn new(name: &str, srs: &str) -> Map {
    	Map{
    		name: name.to_string(),
    		srs: srs.to_string(),
    		extent: Bbox::empty(),
    		layers: Vec::new(),
    	}
    }

    pub fn from_str(file: &str) -> io::Result<Map> {
        let docs = YamlLoader::load_from_str(file).unwrap();
        let doc = &docs[0];
        let hash = doc.as_hash().unwrap();

        let name = hash.get(&Yaml::from_str("name")).unwrap();
        let name_str = name.as_str().unwrap();

        let srs = hash.get(&Yaml::from_str("srs")).unwrap();
        let srs_str = srs.as_str().unwrap();

        let layers = match hash.get(&Yaml::from_str("layers")) {
            Some(layers) => {
                let mut lvec = Vec::new();
                for layer in layers.as_vec().unwrap() {
                    let lh = layer.as_hash().unwrap();
                    let layer = Map::parse_layer(lh);
                    lvec.push(layer);
                }
                lvec
            },
            None => Vec::new()
        };
        
        let extent = match hash.get(&Yaml::from_str("extent")) {
            Some(ext) => {
                match ext.as_vec() {
                    Some(arr) => {
                        // awful.
                        let x0 = arr.get(0).unwrap().as_f64().unwrap();
                        let y0 = arr.get(1).unwrap().as_f64().unwrap();
                        let x1 = arr.get(2).unwrap().as_f64().unwrap();
                        let y1 = arr.get(3).unwrap().as_f64().unwrap();
                        Bbox::new(x0, y0, x1, y1)
                    },
                    None => Bbox::empty()
                }
            },
            None => Bbox::empty()
        };

        Ok(Map{
            name: name_str.to_owned(),
            srs: srs_str.to_owned(),
            extent: extent,
            layers: layers,
        })
    }

    pub fn from_file(path: &Path) -> io::Result<Map> {
        let mut file = try!(File::open(&path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        Map::from_str(&contents)
    }

    fn parse_layer(lh: &yaml::Hash) -> Layer {
        let name = lh.get(&Yaml::from_str("name")).unwrap();
        let name_str = name.as_str().unwrap();

        let style = match lh.get(&Yaml::from_str("style")) {
            Some(s) => s.as_str().unwrap(),
            None => ""
        };

        let source = match lh.get(&Yaml::from_str("source")) {
            Some(s) => s.as_str().unwrap(),
            None => ""
        };

        Layer {
            name: name_str.to_owned(),
            style: style.to_owned(),
            source: source.to_owned(),
        }
    }
}

#[test]
fn it_works() {
    let f = r##"
---
name  : map of los angeles
srs   : +proj=longlat +datum=WGS84 +no_defs
layers:
  - name: los angeles roads
    "##;
    let map = Map::from_str(f).ok().unwrap();
    assert_eq!(map.name, "map of los angeles");
    assert_eq!(map.srs, "+proj=longlat +datum=WGS84 +no_defs");
}