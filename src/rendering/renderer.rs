extern crate cairo;
extern crate geo;
extern crate proj;

use map::Map;
use bbox::{Bbox, LineIter};
use rendering::symbolizers::{PolygonSymbolizer, parse_color};

use self::geo::{Coordinate, Point,ToGeo, Geometry};

use std::io::{Result};
use std::f64::consts::PI;

const DEG_TO_RAD: f64 = PI / 180.;

pub struct CairoRenderer<'a> {
    ctx: &'a cairo::Context,
    map: &'a Map,
}

impl<'a> CairoRenderer<'a> {
    pub fn new(ctx: &'a cairo::Context, map: &'a Map) -> CairoRenderer<'a> {
        CairoRenderer{
            ctx: ctx,
            map: map
        }
    }
    pub fn process_layers(&self) -> Result<()> {
        let ref cr = self.ctx;
        let ref bounds = self.map.extent;
        let ref bounds_rad = bounds.to_rad();
        let t_srs = proj::Proj::new(&self.map.srs).unwrap();

        match parse_color("rgb(211,153,110)") {
            Some(c) => {
                println!("({:?})", c);
                cr.set_source_rgba(c.red as f64, c.green as f64, c.blue as f64, c.alpha as f64);
                cr.paint();
            },
            None => {}
        };

        for layer in self.map.layers.iter() {
            let f_srs = proj::Proj::new("+proj=longlat +datum=WGS84 +no_defs").unwrap();

            let mut bb = Bbox::empty();
            for x in LineIter::new(bounds_rad.min.x, bounds_rad.max.x, 0.001) {
                let Point(coord0) = f_srs.project(&t_srs, Point(Coordinate{x:x, y:bounds_rad.min.y}));
                bb.add_point(coord0);

                let Point(coord1) = f_srs.project(&t_srs, Point(Coordinate{x:x, y:bounds_rad.max.y}));
                bb.add_point(coord1);
            }
            for y in LineIter::new(bounds_rad.min.y, bounds_rad.max.y, 0.001) {
                let Point(coord0) = f_srs.project(&t_srs, Point(Coordinate{y:y, x:bounds_rad.min.x}));
                bb.add_point(coord0);

                let Point(coord1) = f_srs.project(&t_srs, Point(Coordinate{y:y, x:bounds_rad.max.x}));
                bb.add_point(coord1);
            }

            let matrix = get_transform(
                Coordinate{x: bb.min.x, y: bb.max.y}, 
                Coordinate{x: bb.max.x, y: bb.min.y}, 
                1000., 
                1000.);

            // cr.set_source_rgba(0., 0., 0., 1.);
            // for x in LineIter::new(-180., 180., 10.) {
            //     cr.save();
            //     cr.translate(matrix.x0, matrix.y0);
            //     cr.scale(matrix.xx, matrix.yy);
            //     let p = Point(Coordinate{x: x * DEG_TO_RAD, y: -60. * DEG_TO_RAD});
            //     let Point(coord) = f_srs.project(&t_srs, p);
            //     cr.move_to(coord.x, coord.y);
            //     for y in LineIter::new(-60., 85., 1.) {
            //         let p = Point(Coordinate{x: x * DEG_TO_RAD, y: y * DEG_TO_RAD});
            //         let Point(coord) = f_srs.project(&t_srs, p);
            //         cr.line_to(coord.x, coord.y);
            //     }
            //     cr.restore();
            //     cr.stroke();
            // }
            // for y in LineIter::new(-65., 85., 10.) {
            //     cr.save();
            //     cr.translate(matrix.x0, matrix.y0);
            //     cr.scale(matrix.xx, matrix.yy);
            //     let p = Point(Coordinate{x: -180. * DEG_TO_RAD, y: y * DEG_TO_RAD});
            //     let Point(coord) = f_srs.project(&t_srs, p);
            //     cr.move_to(coord.x, coord.y);
            //     for x in LineIter::new(-180., 180., 1.) {
            //         let p = Point(Coordinate{x: x * DEG_TO_RAD, y: y * DEG_TO_RAD});
            //         let Point(coord) = f_srs.project(&t_srs, p);
            //         cr.line_to(coord.x, coord.y);
            //     }
            //     cr.restore();
            //     cr.stroke();
            // }
            
            layer.query(bounds, |feature| {
                let geometry = feature.geometry();
                match geometry.to_geo() {
                    Geometry::Polygon(polygon) => {
                        PolygonSymbolizer::new(polygon, &f_srs, &t_srs).draw(&self.ctx, &matrix)
                    },
                    _ => {}
                }  
            });
        }
        Ok(())
    }
}

pub fn get_transform(b0: Coordinate, c0: Coordinate, pxf: f64, pyf: f64) -> cairo::Matrix {
    let dx = f64::abs(c0.x - b0.x);
    let dy = f64::abs(c0.y - b0.y);
    let x_scale = pxf / dx;
    let y_scale = pyf / dy;
    let scale = f64::min(x_scale, y_scale);
    let (w, h) = (dx * scale, dy * scale);
    let (ox, oy) = ((pxf-w)/2., (pyf-h)/2.);
    matrix_from_rects(b0, c0, Coordinate{x: ox, y: oy}, Coordinate {x: ox + w, y: oy + h})
}

pub fn matrix_from_rects(b0: Coordinate, c0: Coordinate, b1: Coordinate, c1: Coordinate) -> cairo::Matrix {
    let x_scale = (c1.x - b1.x) / (c0.x - b0.x);
    let y_scale = (c1.y - b1.y) / (c0.y - b0.y);
    let x_offset = b1.x - (b0.x * x_scale);
    let y_offset = b1.y - (b0.y * y_scale);
    cairo::Matrix{xx: x_scale, yx: 0., xy: 0., yy: y_scale, x0: x_offset, y0: y_offset}
}

