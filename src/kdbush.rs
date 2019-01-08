extern crate num;
extern crate num_traits;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use conv::prelude::*;
use std::fmt;

use num::Num;
use serde_derive::Deserialize;

pub struct KDBush<T>
where
    T: Coords,
    T::CoordType: Num + PartialOrd,
{
    pub points: Vec<T>,
    // pub index: Box<FnMut(&T) -> Point<T::CoordType>>,
    // pub coords: Vec<Point<T::CoordType>>,
    pub node_size: usize,
    pub ids: Vec<usize>,
}

type TIndex = usize;

pub struct RawCoord<T>(pub T, pub T) where T: Num + PartialOrd;

impl<T> Coords for RawCoord<T> where T: Num + PartialOrd + Copy {
    type CoordType = T;
    fn get_x(&self) -> <RawCoord<T> as Coords>::CoordType {
        self.0
    }

    fn get_y(&self) -> <RawCoord<T> as Coords>::CoordType {
        self.1
    }

    fn get(&self, i: i8) -> <RawCoord<T> as Coords>::CoordType {
        match i {
            0 => self.0,
            _ => self.1,
        }
    }
}

impl<T> fmt::Debug for RawCoord<T> where T: Num + PartialOrd + fmt::Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.0, self.1)
    }
}

pub struct Point<T>(pub T, pub T);

impl<T> Point<T>
where
    T: Num + PartialOrd,
{
    pub fn get(&self, i: i8) -> &T {
        match i {
            0 => &self.0,
            _ => &self.1,
        }
    }
}

impl<T> fmt::Debug for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.0, self.1)
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

impl<T> fmt::Debug for KDBush<T>
where
    T: Coords + fmt::Debug,
    T::CoordType: Num + PartialOrd + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let p: Vec<(T::CoordType, T::CoordType)> = self.points.iter().map(|p| (p.get_x(),p.get_y())).collect();
        p.fmt(formatter)
    }
}

pub trait Coords {
    type CoordType: Num + PartialOrd;
    fn get_x(&self) -> Self::CoordType;
    fn get_y(&self) -> Self::CoordType;
    fn get(&self, i: i8) -> Self::CoordType;
}

impl Coords for City {
    type CoordType = f64;
    fn get_x(&self) -> <City as Coords>::CoordType {
        self.lon
    }
    fn get_y(&self) -> <City as Coords>::CoordType {
        self.lat
    }
    fn get(&self, i: i8) -> <City as Coords>::CoordType {
        match i {
            0 => self.lon,
            _ => self.lat,
        }
    }
}

impl<T> KDBush<T>
where
    T: Coords,
    T::CoordType: Num + PartialOrd,
{
    pub fn new(
        points: Vec<T>,
        // index: Box<FnMut(&T) -> Point<T::CoordType>>,
        node_size: usize,
    ) -> Result<KDBush<T>, std::io::Error> {
        let ids: Vec<usize> = points.iter().enumerate().map(|(i, _)| i).collect();
        // let coords = points
        //     .iter()
        //     .map(|p| -> Point<T::CoordType> { index(p) })
        //     .collect();
        let mut new_kdb = KDBush {
            points: points,
            // index: index,
            // coords: coords,
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
        min_x: &T::CoordType,
        min_y: &T::CoordType,
        max_x: &T::CoordType,
        max_y: &T::CoordType,
        mut result: &mut Vec<TIndex>,
        left: Option<TIndex>,
        right: Option<TIndex>,
        axis: Option<i8>,
    ) where
        T: Coords,
        T::CoordType: Num + PartialOrd,
    {
        if self.points.is_empty() {
            return;
        }

        let left = left.unwrap_or(0);
        let right = right.unwrap_or(self.ids.len() - 1);
        let axis = axis.unwrap_or(0);

        if right - left <= self.node_size {
            (left..right + 1).fold(&mut result, |r, i| {
                let p = &self.points[self.ids[i]];
                // let (x,y) = p.get_coords();
                let x = &p.get_x();
                let y = &p.get_y();
                if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                    r.push(self.ids[i]);
                }
                r
            });
            return;
        }

        let m: TIndex = (left + right) >> 1;
        let x = &self.points[self.ids[m]].get_x();
        let y = &self.points[self.ids[m]].get_y();

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
        qx: T::CoordType,
        qy: T::CoordType,
        r: T::CoordType,
        mut result: &mut Vec<TIndex>,
        left: Option<TIndex>,
        right: Option<TIndex>,
        axis: Option<u8>,
    ) where
        T::CoordType: Num + std::ops::Mul + Copy,
    {
        if self.points.is_empty() {
            return;
        }

        let left = left.unwrap_or(0);
        let right = right.unwrap_or(self.ids.len() - 1);
        let axis = axis.unwrap_or(0);

        let r2 = r * r;

        if right - left <= self.node_size {
            (left..right + 1).fold(&mut result, |r, i| {
                let p = &self.points[self.ids[i]];
                if Self::sq_dist(p.get_x().clone(), p.get_y().clone(), qx.clone(), qy.clone()) <= r2
                {
                    r.push(self.ids[i]);
                }
                r
            });
            return;
        }

        let m = (left + right) >> 1;
        let p = &self.points[self.ids[m]];
        let x = p.get_x();
        let y = p.get_y();

        if KDBush::<T>::sq_dist(x.clone(), y.clone(), qx.clone(), qy.clone()) <= r2 {
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

            let t = self.points[self.ids[k]].get(coord_i);
            let mut i = left;
            let mut j = right;

            self.swap_item(left, k);
            if self.points[self.ids[right]].get(coord_i) > t {
                self.swap_item(left, right);
            }

            while i < j {
                self.swap_item(i, j);
                i += 1;
                j -= 1;

                while self.points[self.ids[i]].get(coord_i) < t {
                    i += 1;
                }
                while self.points[self.ids[j]].get(coord_i) > t {
                    j -= 1;
                }
            }

            if self.points[self.ids[left]].get(coord_i) == t {
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
        // self.coords.swap(i, j);
    }

    fn sq_dist(
        ax: T::CoordType,
        ay: T::CoordType,
        bx: T::CoordType,
        by: T::CoordType,
    ) -> T::CoordType
    where
        T::CoordType: Num + Clone,
        // &'a T::CoordType: Num + std::ops::Mul + std::ops::Add
    {
        let dx = ax - bx;
        let dy = ay - by;
        dx.clone() * dx + dy.clone() * dy
    }
}
