
extern crate gdal;

use self::gdal::vector::{Dataset, Feature, Geometry};
use bbox::Bbox;
use std::path::Path;

pub struct Layer {
    pub name: String,
    pub style: String,
    pub source: String,
}

impl Layer {
    pub fn query<F>(&self, bounds: &Bbox, clj: F) where F: Fn(&Feature) {
    	let mut dataset = Dataset::open(Path::new(&self.source)).unwrap();
    	let layer = dataset.layer(0).unwrap();
        println!("{:?}", bounds.min);
        let spatial_filter = Geometry::bbox(bounds.min.x, bounds.min.y, bounds.max.x, bounds.max.y);
        layer.set_spatial_filter(&spatial_filter);
        for feature in layer.features() {
        	clj(&feature);
        }
    }
}
