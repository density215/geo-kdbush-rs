use std::cmp::Ordering;

use crate::kdbush::{KDBush, Point};
use std::collections::BinaryHeap;

pub const EARTH_RADIUS: f64 = 6137.0;
pub const EARTH_CIRCUMFERENCE: f64 = 40007.0;
pub const RAD: f64 = std::f64::consts::PI / 180.0;

struct Node {
    left: usize,
    right: usize,
    axis: u8,
    // dist: f64,
    min_lng: f64,
    min_lat: f64,
    max_lng: f64,
    max_lat: f64,
}

// TODO:
// We need to preserve
// this struct by making
// refs to it from the Point struct
#[derive(PartialEq)]
struct City {
    name: String,
    country: String,
    altCountry: String,
    muni: String,
    muniSub: String,
    featureClass: String,
    featureCode: String,
    adminCode: String,
    population: u32,
    lat: f64,
    lon: f64,
}

// type Dist = f64;
enum PointOrNode<'a, Point, Node> {
    Point(&'a Point),
    Node(Node),
}

struct PointDist<T>(T, f64);

impl<T> PartialEq for PointDist<T> {
    fn eq(&self, other: &PointDist<T>) -> bool {
        self.1 == other.1
    }
}

impl<T> Eq for PointDist<T> {}

impl<T> Ord for PointDist<T> {
    fn cmp(&self, other: &PointDist<T>) -> Ordering {
        if other.1 >= self.1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl<T> PartialOrd for PointDist<T> {
    fn partial_cmp(&self, other: &PointDist<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new(len: usize) -> Node {
        // an object that represents the top kd-tree node (the whole Earth)
        let node = Node {
            left: 0,         // left index in the kd-tree array
            right: len - 1,  // right index
            axis: 0,         // 0 for longitude axis and 1 for latitude axis
            // dist: 0.0,       // will hold the lower bound of children's distances to the query point
            min_lng: -180.0, // bounding box of the node
            min_lat: -90.0,
            max_lng: 180.0,
            max_lat: 90.0,
        };
        node
    }
}

fn around(
    index: KDBush,
    lng: f64,
    lat: f64,
    max_result: u32,
    max_distance: u32,
    predicate: &Option<Box<Fn(&Point) -> bool>>,
) {
    let mut result = vec![Point];
    let cos_lat = f64::cos(lat * RAD);
    let sin_lat = f64::sin(lat * RAD);
    let mut q = BinaryHeap::new();

    let mut node = Some(Node {
        left: 0,                    // left index in the kd-tree array
        right: index.ids.len() - 1, // right index
        axis: 0,                    // 0 for longitude axis and 1 for latitude axis
        // dist: 0.0, // will hold the lower bound of children's distances to the query point
        min_lng: -180.0, // bounding box of the node
        min_lat: -90.0,
        max_lng: 180.0,
        max_lat: 90.0,
    });

    while node.is_some() {
        let left;
        let right;
        match &node {
            Some(node) => {
                right = node.right;
                left = node.left;
            }
            _ => {
                break;
            }
        }

        if right - left <= index.node_size {
            (left..right).for_each(|i: usize| {
                let item = &index.points[index.ids[i]];
                let predicate_check = match predicate {
                    None => true,
                    Some(predicate) => predicate(&item),
                };
                if predicate_check {
                    q.push(PointDist(
                        PointOrNode::Point(item),
                        great_circle_dist(
                            lng,
                            lat,
                            item.get(0).into(),
                            item.get(1).into(),
                            cos_lat,
                            sin_lat,
                        ),
                    ));
                }
            })
        } else {
            let m = (left + right) >> 1;
            let mid_lng = index.points[m].0;
            let mid_lat = index.points[m].1;
            let item = &index.points[index.ids[m]];
            let predicate_check = match predicate {
                None => true,
                Some(predicate) => predicate(&item),
            };
            if predicate_check {
                q.push(PointDist(
                    PointOrNode::Point(item),
                    great_circle_dist(lng, lat, mid_lng.into(), mid_lat.into(), cos_lat, sin_lat),
                ))
            }

            match &node {
                Some(node) => {
                    let next_axis = (node.axis + 1) % 2;

                    let left_node = Node {
                        left: left,
                        right: m - 1,
                        axis: next_axis,
                        min_lng: node.min_lng,
                        min_lat: node.min_lat,
                        max_lng: if node.axis == 0 {
                            mid_lng.into()
                        } else {
                            node.max_lng.into()
                        },
                        max_lat: if node.axis == 0 {
                            mid_lat.into()
                        } else {
                            node.max_lat.into()
                        },
                        // dist: 0.0,
                    };

                    let right_node = Node {
                        left: m + 1,
                        right: right,
                        axis: next_axis,
                        min_lng: if node.axis == 0 {
                            mid_lng.into()
                        } else {
                            node.min_lng.into()
                        },
                        min_lat: if node.axis == 1 {
                            mid_lat.into()
                        } else {
                            node.min_lat.into()
                        },
                        max_lng: node.max_lng.into(),
                        max_lat: node.max_lat.into(),
                        // dist: 0.0,
                    };

                    let left_node_dist = box_dist(lng, lat, Box::new(&left_node), cos_lat, sin_lat);
                    let right_node_dist =
                        box_dist(lng, lat, Box::new(&right_node), cos_lat, sin_lat);
                    q.push(PointDist(PointOrNode::Node(left_node), left_node_dist));
                    q.push(PointDist(PointOrNode::Node(right_node), right_node_dist));
                }
                _ => {
                    break;
                } // can't happen
            };
        }
    }
}

fn box_dist(lng: f64, lat: f64, node: Box<&Node>, cos_lat: f64, sin_lat: f64) -> f64 {
    if lng >= node.min_lng && lng <= node.max_lng {
        match lat {
            lat if lat <= node.min_lat => EARTH_CIRCUMFERENCE * (node.min_lat - lat) / 360.0,
            lat if lat >= node.max_lat => EARTH_CIRCUMFERENCE * (lat - node.max_lat) / 360.0,
            _ => 0.0,
        };
    }

    let closest_lng =
        if (node.min_lng - lng + 360.0) % 360.0 <= (lng - node.max_lng + 360.0) % 360.0 {
            node.min_lng
        } else {
            node.max_lng
        };
    let cos_lng_delta = f64::cos((closest_lng - lng) * RAD);
    let extremum_lat = f64::atan(sin_lat / (cos_lat * cos_lng_delta)) / RAD;

    let mut d = f64::max(
        great_circle_dist_part(node.min_lat, cos_lat, sin_lat, cos_lng_delta),
        great_circle_dist_part(node.max_lat, cos_lat, sin_lat, cos_lng_delta),
    );

    if extremum_lat > node.min_lat && extremum_lat < node.max_lat {
        d = f64::max(
            d,
            great_circle_dist_part(extremum_lat, cos_lat, sin_lat, cos_lng_delta),
        )
    }

    EARTH_RADIUS * f64::acos(d)
}

fn great_circle_dist(lng: f64, lat: f64, lng2: f64, lat2: f64, cos_lat: f64, sin_lat: f64) -> f64 {
    let cos_lng_delta = f64::cos((lng2 - lng) * RAD);
    EARTH_RADIUS * f64::acos(great_circle_dist_part(
        lat2,
        cos_lat,
        sin_lat,
        cos_lng_delta,
    ))
}

fn great_circle_dist_part(lat: f64, cos_lat: f64, sin_lat: f64, cos_lng_delta: f64) -> f64 {
    let d = sin_lat * f64::sin(lat * RAD) + cos_lat * f64::cos(lat * RAD) * cos_lng_delta;
    f64::min(d, 1.0)
}

fn distance(lng: f64, lat: f64, lng2: f64, lat2: f64) -> f64 {
    great_circle_dist(
        lng,
        lat,
        lng2,
        lat2,
        f64::cos(lat * RAD),
        f64::sin(lat * RAD),
    )
}
