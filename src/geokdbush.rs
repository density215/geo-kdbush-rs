extern crate num;
extern crate num_traits;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

use crate::kdbush::{Coords, KDBush};
use num::{Float, NumCast};
use num_traits::FloatConst;

pub const EARTH_RADIUS: f64 = 6137.0;
pub const EARTH_CIRCUMFERENCE: f64 = 40007.0;
pub const RAD: f64 = std::f64::consts::PI / 180.0;

fn earth_radius<T: Float + FloatConst>() -> T
where
    T: std::ops::Div<Output = T>,
{
    NumCast::from(6137.0).unwrap()
}

fn earth_circumference<T: Float + FloatConst>() -> T
where
    T: std::ops::Div<Output = T>,
{
    NumCast::from(40007.0).unwrap()
}

fn rad<T: Float + FloatConst>() -> T
where
    T: std::ops::Div<Output = T>,
{
    T::PI() / NumCast::from(180.0).unwrap()
}

struct Node<T>
where
    T: Float + FloatConst + PartialOrd,
{
    left: usize,
    right: usize,
    axis: u8,
    // dist: f64,
    min_lng: T,
    min_lat: T,
    max_lng: T,
    max_lat: T,
}

// type Dist = f64;
enum PointOrNode<'a, T>
where
    T: Coords,
    T::CoordType: Float + FloatConst + PartialOrd,
{
    Point(&'a T),
    Node(Node<<T as Coords>::CoordType>),
}

impl<'a, T> fmt::Debug for Node<T>
where
    T: fmt::Display + Float + FloatConst + PartialOrd,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "({},{},{},{},{},{})",
            self.left, self.right, self.min_lat, self.min_lng, self.max_lat, self.max_lng
        )
    }
}

struct PointDist<S, T>(S, T)
where
    // S: Coords,
    T: Float + FloatConst + PartialOrd;

impl<S, T> PartialEq for PointDist<S, T>
where
    T: Float + FloatConst + PartialOrd,
    // S: Coords,
{
    fn eq(&self, other: &PointDist<S, T>) -> bool {
        self.1 == other.1
    }
}

impl<S, T> Eq for PointDist<S, T> where T: Float + FloatConst + PartialOrd // S: Coords,,
{
}

impl<S, T> Ord for PointDist<S, T>
where
    T: Float + FloatConst + PartialOrd,
    // S: Coords,
{
    fn cmp(&self, other: &PointDist<S, T>) -> Ordering {
        if other.1 > self.1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<S, T> PartialOrd for PointDist<S, T>
where
    T: Float + FloatConst + PartialOrd,
    // S: Coords,
{
    fn partial_cmp(&self, other: &PointDist<S, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// impl Node {
//     pub fn new(len: usize) -> Node {
//         // an object that represents the top kd-tree node (the whole Earth)
//         let node = Node {
//             left: 0,        // left index in the kd-tree array
//             right: len - 1, // right index
//             axis: 0,        // 0 for longitude axis and 1 for latitude axis
//             // dist: 0.0,       // will hold the lower bound of children's distances to the query point
//             min_lng: -180.0, // bounding box of the node
//             min_lat: -90.0,
//             max_lng: 180.0,
//             max_lat: 90.0,
//         };
//         node
//     }
// }

pub fn around<'a, T>(
    index: &'a KDBush<T>,
    lng: T::CoordType,
    lat: T::CoordType,
    max_results: Option<usize>,
    max_distance: Option<T::CoordType>,
    predicate: &Option<Box<Fn(&T) -> bool>>,
) -> Vec<&'a T>
where
    T: fmt::Debug + Coords,
    T::CoordType: Float + PartialOrd + FloatConst + fmt::Debug + fmt::Display,
{
    let mut result = vec![];
    let cos_lat = T::CoordType::cos(lat * rad::<T::CoordType>());
    let sin_lat = T::CoordType::sin(lat * rad::<T::CoordType>());
    let mut q = BinaryHeap::new();

    // an object that represents the top kd-tree node (the whole Earth)
    let mut point_or_node = PointOrNode::Node(Node::<T::CoordType> {
        left: 0,                    // left index in the kd-tree array
        right: index.ids.len() - 1, // right index
        axis: 0,                    // 0 for longitude axis and 1 for latitude axis
        // dist: 0.0, // will hold the lower bound of children's distances to the query point
        min_lng: NumCast::from(-180.0).unwrap(), // bounding box of the node
        min_lat: NumCast::from(-90.0).unwrap(),
        max_lng: NumCast::from(180.0).unwrap(),
        max_lat: NumCast::from(90.0).unwrap(),
    });

    'tree: loop {
        let left;
        let right;
        if let PointOrNode::Node(node) = &point_or_node {
            right = node.right;
            left = node.left
        } else {
            panic!("No node in current enum.");
        };

        println!("left:{:?},right: {:?}", left, right);
        println!("node_size: {:?}", index.node_size);
        if (right - left) <= index.node_size {
            // leaf node
            println!("fill heap");
            (left..(right + 1)).for_each(|i: usize| {
                let item = &index.points[index.ids[i]];
                let predicate_check = match predicate {
                    None => true,
                    Some(predicate) => predicate(item),
                };
                let dist = great_circle_dist(
                    lng,
                    lat,
                    // index.coords[i].get(0).into(),
                    NumCast::from(item.get_x()).unwrap(),
                    // index.coords[i].get(1).into(),
                    NumCast::from(item.get_y()).unwrap(),
                    cos_lat,
                    sin_lat,
                );
                println!("leaf to heap {:?}", item);
                println!("{:?}", dist);
                if predicate_check {
                    q.push(PointDist(PointOrNode::Point(item), dist));
                }
            })
        } else {
            // not a leaf node (has children). branch.
            println!("branch node");
            let m = (left + right) >> 1;
            // let mid_lng = index.coords[m].get(0);
            let mid_lng = index.points[index.ids[m]].get_x();
            // let mid_lat = index.coords[m].get(1);
            let mid_lat = index.points[index.ids[m]].get_y();

            let item = &index.points[index.ids[m]];
            let predicate_check = match predicate {
                None => true,
                Some(predicate) => predicate(item),
            };
            if predicate_check {
                let dist = great_circle_dist(lng, lat, mid_lng, mid_lat, cos_lat, sin_lat);
                println!("branch to heap");
                println!("{:?}", dist);
                println!("{:?}", item);
                q.push(PointDist(PointOrNode::Point(item), dist))
            }

            if let PointOrNode::Node(node) = point_or_node {
                let next_axis = (node.axis + 1) % 2;

                let left_node = Node::<<T as Coords>::CoordType> {
                    left: left,
                    right: m - 1,
                    axis: next_axis,
                    min_lng: NumCast::from(node.min_lng).unwrap(),
                    min_lat: NumCast::from(node.min_lat).unwrap(),
                    max_lng: if node.axis == 0 {
                        NumCast::from(mid_lng).unwrap()
                    } else {
                        NumCast::from(node.max_lng).unwrap()
                    },
                    max_lat: if node.axis == 1 {
                        NumCast::from(mid_lat).unwrap()
                    } else {
                        NumCast::from(node.max_lat).unwrap()
                    },
                    // dist: 0.0,
                };

                let right_node = Node::<<T as Coords>::CoordType> {
                    left: m + 1,
                    right: right,
                    axis: next_axis,
                    min_lng: if node.axis == 0 {
                        NumCast::from(mid_lng).unwrap()
                    } else {
                        NumCast::from(node.min_lng).unwrap()
                    },
                    min_lat: if node.axis == 1 {
                        NumCast::from(mid_lat).unwrap()
                    } else {
                        NumCast::from(node.min_lat).unwrap()
                    },
                    max_lng: NumCast::from(node.max_lng).unwrap(),
                    max_lat: NumCast::from(node.max_lat).unwrap(),
                    // dist: 0.0,
                };

                let left_node_dist = box_dist(lng, lat, Box::new(&left_node), cos_lat, sin_lat);
                let right_node_dist = box_dist(lng, lat, Box::new(&right_node), cos_lat, sin_lat);
                q.push(PointDist(PointOrNode::Node(left_node), left_node_dist));
                q.push(PointDist(PointOrNode::Node(right_node), right_node_dist));
                println!("{:?}", q.len());
            }
        }

        while q.len() > 0 && q.peek().is_some() {
            if let PointOrNode::Point(_) = q.peek().unwrap().0 {
                // a leaf node was found
                let candidate = q.pop().unwrap();
                if max_distance.is_some() && candidate.1 > max_distance.unwrap() {
                    println!("max distance reached");
                    return result;
                }
                if let PointOrNode::Point(point) = candidate.0 {
                    println!("candidate");
                    println!("point :\t{:?}", point);
                    println!("dist :\t{:?}", candidate.1);
                    result.push(point);
                } else {
                    println!("wut?");
                    // if let PointOrNode::Node(node) = candidate.0 {
                    //     println!("{:?}", node);
                    // }
                }

                if max_results.is_some() && result.len() == max_results.unwrap() {
                    println!("stop results.");
                    return result;
                }
            } else {
                // no point found, this is a branch node
                break;
            };
        }

        println!("heap length : \t{:?}", q.len());
        let node_dp = q.pop();

        match node_dp.unwrap() {
            PointDist(p, _) => {
                point_or_node = p;
            }
        };
    }

    result
}

fn box_dist<T>(lng: T, lat: T, node: Box<&Node<T>>, cos_lat: T, sin_lat: T) -> T
where
    T: Float + FloatConst + PartialOrd,
{
    let three60 = NumCast::from(360.0).unwrap();
    if lng >= node.min_lng && lng <= node.max_lng {
        let lat = match lat {
            lat if lat <= node.min_lat => {
                earth_circumference::<T>() * (node.min_lat - lat) / three60
            }
            lat if lat >= node.max_lat => {
                earth_circumference::<T>() * (lat - node.max_lat) / three60
            }
            _ => NumCast::from(0.0).unwrap(),
        };
        return lat;
    }

    let closest_lng =
        if (node.min_lng - lng + three60) % three60 <= (lng - node.max_lng + three60) % three60 {
            node.min_lng
        } else {
            node.max_lng
        };
    let cos_lng_delta = T::cos((closest_lng - lng) * rad::<T>());
    let extremum_lat = T::atan(sin_lat / (cos_lat * cos_lng_delta)) / rad::<T>();

    let mut d = T::max(
        great_circle_dist_part(node.min_lat, cos_lat, sin_lat, cos_lng_delta),
        great_circle_dist_part(node.max_lat, cos_lat, sin_lat, cos_lng_delta),
    );

    if extremum_lat > node.min_lat && extremum_lat < node.max_lat {
        d = T::max(
            d,
            great_circle_dist_part(extremum_lat, cos_lat, sin_lat, cos_lng_delta),
        )
    }

    earth_radius::<T>() * T::acos(d)
}

fn great_circle_dist<T>(lng: T, lat: T, lng2: T, lat2: T, cos_lat: T, sin_lat: T) -> T
where
    T: Float + PartialOrd + FloatConst + std::ops::Mul<Output = T>,
{
    let cos_lng_delta = T::cos((lng2 - lng) * rad::<T>());
    earth_radius::<T>() * T::acos(great_circle_dist_part(
        lat2,
        cos_lat,
        sin_lat,
        cos_lng_delta,
    ))
}

fn great_circle_dist_part<T>(lat: T, cos_lat: T, sin_lat: T, cos_lng_delta: T) -> T
where
    T: Float + PartialOrd + FloatConst,
{
    let d = sin_lat * T::sin(lat * rad::<T>()) + cos_lat * T::cos(lat * rad::<T>()) * cos_lng_delta;
    T::min(d, num::one::<T>())
}

pub fn distance<T>(lng: T, lat: T, lng2: T, lat2: T) -> T
where
    T: Float + PartialOrd + FloatConst,
{
    great_circle_dist(
        lng,
        lat,
        lng2,
        lat2,
        T::cos(lat * rad::<T>()),
        T::sin(lat * rad::<T>()),
    )
}
