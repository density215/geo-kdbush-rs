extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use conv::prelude::*;
use std::fmt;

use serde_derive::Deserialize;

pub struct KDBush<T> {
    pub points: Vec<T>,
    pub index: Box<FnMut(&T) -> Point>,
    pub coords: Vec<Point>,
    pub node_size: usize,
    pub ids: Vec<usize>,
}

type TIndex = usize;

type TNumber = GNumber<f64>;
type RawCoord = (TNumber, TNumber);

type GNumber<T> = T;

pub struct Point(pub TNumber, pub TNumber);

impl Point {
    pub fn get(&self, i: i8) -> TNumber {
        match i {
            0 => self.0,
            _ => self.1,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct City {
    pub name: String,
    country: String,
    altCountry: String,
    muni: String,
    muniSub: String,
    featureClass: String,
    featureCode: String,
    adminCode: String,
    pub population: u32,
    pub lat: f64,
    pub lon: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.0, self.1)
    }
}

impl<T> fmt::Debug for KDBush<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.coords[..].fmt(formatter)
    }
}

pub trait CoordsGetter<T> {
    fn get_coords(&self) -> Box<FnMut(&T) -> Point>;

    // fn get_coords(&self, p: &T) -> Point;

    // impl<'a> CoordsGetter<Point> for KDBush<'a, (TNumber, TNumber)> {
    //     fn get_coords(p: &Point) -> Point {
    //         Point(p.0, p.1)
    //     }
}

pub trait Coords {
    fn get_x(&self) -> &TNumber;
    fn get_y(&self) -> &TNumber;
}

impl<T> CoordsGetter<RawCoord> for KDBush<T> {
    fn get_coords(&self) -> Box<FnMut(&RawCoord) -> Point> {
        Box::new(|p: &RawCoord| Point(p.0, p.1))
    }
    // fn get_coords(&self, p: &RawCoord) -> Point {
    //     Point(p.0, p.1)
    // }
}

impl<T> CoordsGetter<City> for KDBush<T>
// where
//     &'static KDBush<City>: CoordsGetter<T>,
{
    fn get_coords(&self) -> Box<FnMut(&City) -> Point> {
        Box::new(|c: &City| Point(c.lon, c.lat))
    }
    // fn get_coords(&self, c: &City) -> Point {
    //     Point(c.lon, c.lat)
    // }
}

impl Coords for City {
    fn get_x(&self) -> &TNumber {
        &self.lon
    }
    fn get_y(&self) -> &TNumber {
        &self.lat
    }
}

impl Coords for RawCoord {
    fn get_x(&self) -> &TNumber {
        &self.0
    }

    fn get_y(&self) -> &TNumber {
        &self.1 
    }
}

impl<T> KDBush<T>
// where
//     KDBush<T>: CoordsGetter<T>,
{
    pub fn new(
        points: Vec<T>,
        mut index: Box<FnMut(&T) -> Point>,
        node_size: usize,
    ) -> Result<KDBush<T>, std::io::Error> {
        let ids: Vec<usize> = points.iter().enumerate().map(|(i, _)| i).collect();
        let coords = points.iter().map(|p| -> Point { index(p) }).collect();
        let mut new_kdb = KDBush {
            points: points,
            index: index,
            coords: coords,
            node_size: node_size,
            ids: ids,
        };
        let l = new_kdb.ids.len();
        // new_kdb.fill_cords();

        match l {
            l if l >= 1 => {
                new_kdb.sort_kd(0, l - 1, 0);
                Ok(new_kdb)
            }
            _ => Ok(new_kdb),
        }
    }

    // fn fill_cords(&mut self) {
    //     // let mut coords_get: Box<FnMut(&T) -> Point> = Self::get_coords(&self);
    //     let mut coords_get = &self.index;

    //     self.coords = self
    //         .points
    //         .iter()
    //         .map(|p| -> Point { coords_get(p) })
    //         .collect();
    // }

    pub fn range(
        &self,
        min_x: TNumber,
        min_y: TNumber,
        max_x: TNumber,
        max_y: TNumber,
        mut result: &mut Vec<TIndex>,
        left: Option<TIndex>,
        right: Option<TIndex>,
        axis: Option<i8>,
    ) {
        if self.coords.is_empty() {
            return;
        }

        let left = left.unwrap_or(0);
        let right = right.unwrap_or(self.ids.len() - 1);
        let axis = axis.unwrap_or(0);

        if right - left <= self.node_size {
            (left..right + 1).fold(&mut result, |r, i| {
                let p = &self.coords[i];
                // let (x,y) = p.get_coords();
                let x = p.get(0);
                let y = p.get(1);
                if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                    r.push(self.ids[i]);
                }
                r
            });
            return;
        }

        let m: TIndex = (left + right) >> 1;
        let x = self.coords[m].get(0);
        let y = self.coords[m].get(1);

        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            result.push(self.ids[m]);
        };

        if if axis == 0 { min_x <= x } else { min_y <= y } {
            self.range(
                min_x,
                min_y,
                max_x,
                max_y,
                &mut result,
                Some(left),
                Some(m - 1),
                Some((axis + 1) % 2),
            );
        }

        if if axis == 0 { max_x >= x } else { max_y >= y } {
            self.range(
                min_x,
                min_y,
                max_x,
                max_y,
                &mut result,
                Some(m + 1),
                Some(right),
                Some((axis + 1) % 2),
            );
        }
    }

    pub fn within(
        &self,
        qx: TNumber,
        qy: TNumber,
        r: TNumber,
        mut result: &mut Vec<TIndex>,
        left: Option<TIndex>,
        right: Option<TIndex>,
        axis: Option<u8>,
    ) {
        if self.coords.is_empty() {
            return;
        }

        let left = left.unwrap_or(0);
        let right = right.unwrap_or(self.ids.len() - 1);
        let axis = axis.unwrap_or(0);

        let r2 = r * r;

        if right - left <= self.node_size {
            (left..right + 1).fold(&mut result, |r, i| {
                let p = &self.coords[i];
                if KDBush::<T>::sq_dist(p.get(0), p.get(1), qx, qy) <= r2 {
                    r.push(self.ids[i]);
                }
                r
            });
            return;
        }

        let m = (left + right) >> 1;
        let p = &self.coords[m];
        let x = p.get(0);
        let y = p.get(1);

        if KDBush::<T>::sq_dist(x, y, qx, qy) <= r2 {
            result.push(self.ids[m]);
        }

        if if axis == 0 { qx - r <= x } else { qy - r <= y } {
            self.within(
                qx,
                qy,
                r,
                result,
                Some(left),
                Some(m - 1),
                Some((axis + 1) % 2),
            );
        }
        if if axis == 0 { qx + r >= x } else { qy + r >= y } {
            self.within(
                qx,
                qy,
                r,
                result,
                Some(m + 1),
                Some(right),
                Some((axis + 1) % 2),
            );
        }
    }

    fn sort_kd(&mut self, left: TIndex, right: TIndex, axis: u8) {
        if right - left <= self.node_size {
            return;
        }
        let m: TIndex = (left + right) >> 1;
        if axis == 0 {
            &mut self.select(0, m, left, right);
        } else {
            &mut self.select(1, m, left, right);
        }
        &self.sort_kd(left, m - 1, (axis + 1) % 2);
        &self.sort_kd(m + 1, right, (axis + 1) % 2);
    }

    fn select(&mut self, coord_i: i8, k: TIndex, mut left: TIndex, mut right: TIndex) {
        while right > left {
            if right - left > 600 {
                let n: f64 = f64::value_from(right - left + 1).unwrap();
                let m = f64::value_from(k - left + 1).unwrap();
                let z = f64::ln(n);
                let s: f64 = 0.5 * f64::exp(2.0 * z / 3.0);
                let r: f64 = f64::value_from(k).unwrap() - m * s / n
                    + 0.5
                        * f64::sqrt(z * s * (1.0 - s / n))
                        * (if 2.0 * m < n { -1.0 } else { 1.0 });
                self.select(
                    coord_i,
                    k,
                    usize::max(left, r as usize),
                    usize::min(right, (r + s) as usize),
                );
            };

            let t = self.coords[k].get(coord_i);
            let mut i = left;
            let mut j = right;

            self.swap_item(left, k);
            if self.coords[right].get(coord_i) > t {
                self.swap_item(left, right);
            }

            while i < j {
                self.swap_item(i, j);
                i += 1;
                j -= 1;

                while self.coords[i].get(coord_i) < t {
                    i += 1;
                }
                while self.coords[j].get(coord_i) > t {
                    j -= 1;
                }
            }

            if self.coords[left].get(coord_i) == t {
                self.swap_item(left, j);
            } else {
                j += 1;
                self.swap_item(j, right);
            }

            if j <= k {
                left = j + 1;
            }
            if k <= j {
                right = j - 1;
            }
        }
    }

    fn swap_item(&mut self, i: usize, j: usize) {
        self.ids.swap(i, j);
        self.coords.swap(i, j);
    }

    fn sq_dist(ax: TNumber, ay: TNumber, bx: TNumber, by: TNumber) -> TNumber {
        let dx = ax - bx;
        let dy = ay - by;
        dx * dx + dy * dy
    }
}
