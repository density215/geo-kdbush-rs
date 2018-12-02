use conv::prelude::*;
use std::fmt;

pub struct KDBush {
    pub points: Vec<Point>,
    pub node_size: usize,
    pub ids: Vec<usize>,
}

type TIndex = usize;

type TNumber = i16;

pub struct Point(pub TNumber, pub TNumber);

impl Point {
    pub fn get(&self, i: i8) -> TNumber {
        match i {
            0 => self.0,
            _ => self.1,
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.0, self.1)
    }
}

impl fmt::Debug for KDBush {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.points[..].fmt(formatter)
    }
}

impl KDBush {
    pub fn new(
        points: Vec<(TNumber, TNumber)>,
        node_size: usize,
    ) -> Result<KDBush, std::io::Error> {
        let mut new_kdb = KDBush {
            points: points.iter().map(|p| Point(p.0, p.1)).collect(),
            node_size: node_size,
            ids: points.iter().enumerate().map(|(i, _)| i).collect(),
        };
        let l = new_kdb.ids.len();

        match l {
            l if l >= 1 => {
                new_kdb.sort_kd(0, l - 1, 0);
                Ok(new_kdb)
            }
            _ => Ok(new_kdb),
        }
    }

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
        if self.points.is_empty() {
            return;
        }

        let left = left.unwrap_or(0);
        let right = right.unwrap_or(self.ids.len() - 1);
        let axis = axis.unwrap_or(0);

        if right - left <= self.node_size {
            (left..right + 1).fold(&mut result, |r, i| {
                let p = &self.points[i];
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
        let x = self.points[m].get(0);
        let y = self.points[m].get(1);

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
        if self.points.is_empty() {
            return;
        }

        let left = left.unwrap_or(0);
        let right = right.unwrap_or(self.ids.len() - 1);
        let axis = axis.unwrap_or(0);

        let r2 = r * r;

        if right - left <= self.node_size {
            (left..right + 1).fold(&mut result, |r, i| {
                let p = &self.points[i];
                if KDBush::sq_dist(p.get(0), p.get(1), qx, qy) <= r2 {
                    r.push(self.ids[i]);
                }
                r
            });
            return;
        }

        let m = (left + right) >> 1;
        let p = &self.points[m];
        let x = p.get(0);
        let y = p.get(1);

        if KDBush::sq_dist(x, y, qx, qy) <= r2 {
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

            let t = self.points[k].get(coord_i);
            let mut i = left;
            let mut j = right;

            self.swap_item(left, k);
            if self.points[right].get(coord_i) > t {
                self.swap_item(left, right);
            }

            while i < j {
                self.swap_item(i, j);
                i += 1;
                j -= 1;

                while self.points[i].get(coord_i) < t {
                    i += 1;
                }
                while self.points[j].get(coord_i) > t {
                    j -= 1;
                }
            }

            if self.points[left].get(coord_i) == t {
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
        self.points.swap(i, j);
    }

    fn sq_dist(ax: TNumber, ay: TNumber, bx: TNumber, by: TNumber) -> TNumber {
        let dx = ax - bx;
        let dy = ay - by;
        dx * dx + dy * dy
    }
}
