extern crate geo;

use self::geo::{Coordinate};
use std::f64::consts::PI;

pub struct Bbox {
    pub min: Coordinate,
    pub max: Coordinate,
    empty: bool
}

const DEG_TO_RAD: f64 = PI / 180.;

impl Bbox {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Bbox {
        Bbox{ 
            empty: false,
            min: Coordinate{x: f64::min(x0, x1), y: f64::min(y0,y1) },
            max: Coordinate{x: f64::max(x0, x1), y: f64::max(y0,y1) },
        }
    }

    pub fn empty() -> Bbox {
        Bbox{ empty: true, min: Coordinate{x: 0., y:0.}, max: Coordinate{x:0., y:0.} }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn add_point(&mut self, coord: Coordinate) {
        if self.empty {
            self.min = coord;
            self.max = coord;
            self.empty = false;
        } else {
            self.min.x = f64::min(self.min.x, coord.x);
            self.min.y = f64::min(self.min.y, coord.y);
            self.max.x = f64::max(self.max.x, coord.x);
            self.max.y = f64::max(self.max.y, coord.y);
        }
    }

    pub fn to_rad(&self) -> Bbox {
        Bbox{ 
            empty: false,
            min: Coordinate{x: self.min.x * DEG_TO_RAD, y: self.min.y * DEG_TO_RAD },
            max: Coordinate{x: self.max.x * DEG_TO_RAD, y: self.max.y * DEG_TO_RAD },
        }
    }
}

pub struct LineIter {
    curr: f64,
    end: f64,
    step: f64,
}

impl LineIter {
    pub fn new(min: f64, max: f64, step: f64) -> LineIter {
        LineIter{curr: min, end: max, step: step}
    }
}
impl Iterator for LineIter {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        self.curr += self.step;
        if self.curr < self.end {
            Some(self.curr)
        } else {
            None
        }
    }
}

#[test]
fn empty_works() {
    let mut bb = Bbox::empty();
    assert!(bb.empty, "should be empty");

    bb.add_point(Coordinate{x: 1., y: 1.});
    assert!(!bb.empty, "should not be empty");

    assert_eq!(bb.min.x, 1.);
    assert_eq!(bb.min.y, 1.);
    assert_eq!(bb.max.x, 1.);
    assert_eq!(bb.max.y, 1.);
}

#[test]
fn new_works() {
    let mut bb = Bbox::new(1., 1., 1., 1.);
    assert!(!bb.empty, "should not be empty");

    assert_eq!(bb.min.x, 1.);
    assert_eq!(bb.min.y, 1.);
    assert_eq!(bb.max.x, 1.);
    assert_eq!(bb.max.y, 1.);

    bb.add_point(Coordinate{x: -10., y: 2.});
    assert_eq!(bb.min.x, -10.);
    assert_eq!(bb.min.y, 1.);
    assert_eq!(bb.max.x, 1.);
    assert_eq!(bb.max.y, 2.);
}