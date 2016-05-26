extern crate cairo;
extern crate geo;
extern crate proj;
extern crate cssparser;

use self::cssparser::{Parser, Color, RGBA};

use self::geo::{Coordinate, Point};
use std::f64::consts::PI;

const DEG_TO_RAD: f64 = PI / 180.;

pub struct PolygonSymbolizer<'a> {
    f_srs: &'a proj::Proj,
    t_srs: &'a proj::Proj,
    polygon: geo::Polygon,
}

pub fn parse_color(s: &str) -> Option<RGBA> {
    let mut parser = Parser::new(s);
    match Color::parse(&mut parser) {
        Ok(color) => match color {
            Color::CurrentColor => None,
            Color::RGBA(rgba) => Some(rgba)
        },
        Err(_) => None
    }
}

impl<'a> PolygonSymbolizer<'a> {
    pub fn new(p: geo::Polygon, f_srs: &'a proj::Proj, t_srs: &'a proj::Proj) -> PolygonSymbolizer<'a> {
        PolygonSymbolizer{polygon: p, f_srs: f_srs, t_srs: t_srs}
    }

    pub fn draw<'b>(&'a self, cr: &'b cairo::Context, matrix: &cairo::Matrix) {
        match parse_color("#c8cfc8") {
            Some(c) => {
                cr.set_source_rgba(c.red as f64, c.green as f64, c.blue as f64, c.alpha as f64);
            },
            None => {}
        };
        
        let geo::Polygon(ref outer_ls, ref holes) = self.polygon;
        let tf = |&p| {
            let Point(coord) = p;
            self.f_srs.project(&self.t_srs, Point(Coordinate {x: coord.x * DEG_TO_RAD, y: coord.y * DEG_TO_RAD}))
        };
        
        let &geo::LineString(ref outer) = outer_ls;
        let mut iter = outer.into_iter().map(&tf);
        cr.save();

        let Point(coord) = iter.next().unwrap();
        cr.translate(matrix.x0, matrix.y0);
        cr.scale(matrix.xx, matrix.yy);
        cr.move_to(coord.x, coord.y);
        for Point(coord) in iter {
            cr.line_to(coord.x, coord.y);
        }
        cr.set_fill_rule(cairo::FillRule::EvenOdd);
        for hole_ls in holes {
            let &geo::LineString(ref hole) = hole_ls;
            let mut iter = hole.into_iter().map(&tf);
            let Point(coord) = iter.next().unwrap();
            cr.translate(matrix.x0, matrix.y0);
            cr.scale(matrix.xx, matrix.yy);
            cr.move_to(coord.x, coord.y);
            for Point(coord) in iter {
                cr.line_to(coord.x, coord.y);
            }
        }

        cr.restore();
        cr.stroke();
        // cr.fill();
    }
}


