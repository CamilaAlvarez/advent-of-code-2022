use std::collections::HashMap;

use super::distance::Point;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    ImpossibleBeaconLocation,
    Beacon,
    Sensor,
}
struct MapPosition {
    location: Location,
    position: Point,
}

pub struct SensorsReachMap {
    sensor_map: HashMap<i32, Vec<MapPosition>>,
}

pub struct Beacon {
    position: Point,
}
pub struct Sensor {
    position: Point,
    closest_beacon: Beacon,
}

impl SensorsReachMap {
    pub fn new(sensors: &Vec<Sensor>) -> Self {
        let mut sensor_map = HashMap::new();
        Self::fill_map(&mut sensor_map, sensors);
        Self { sensor_map }
    }
    pub fn number_no_possible_beacon_location(&self, row: i32) -> Option<usize> {
        if let Some(row) = self.sensor_map.get(&row) {
            return Some(
                row.iter()
                    .filter(|map_position| {
                        map_position.location == Location::ImpossibleBeaconLocation
                    })
                    .collect::<Vec<_>>()
                    .len(),
            );
        }
        None
    }
    fn fill_map(sensor_map: &mut HashMap<i32, Vec<MapPosition>>, sensors: &Vec<Sensor>) {
        for sensor in sensors.iter() {
            let points = sensor.points_with_impossible_beacons();
            Self::set_value_in_map(sensor_map, sensor.position, Location::Sensor);
            Self::set_value_in_map(sensor_map, sensor.closest_beacon.position, Location::Beacon);
            for point in points.iter() {
                Self::set_value_in_map(
                    sensor_map,
                    point.clone(),
                    Location::ImpossibleBeaconLocation,
                );
            }
        }
    }
    fn set_value_in_map(
        sensor_map: &mut HashMap<i32, Vec<MapPosition>>,
        position: Point,
        value: Location,
    ) {
        let map_position = MapPosition {
            location: value,
            position: position,
        };
        if let Some(row) = sensor_map.get_mut(&position.y()) {
            if !row.contains(&map_position) {
                row.push(map_position);
            }
        } else {
            sensor_map.insert(position.y(), vec![]);
        }
    }
}

impl Beacon {
    pub fn new(x: i32, y: i32) -> Self {
        Self::new_with_point(Point::new(x, y))
    }
    pub fn new_with_point(point: Point) -> Self {
        Self { position: point }
    }
}

impl Sensor {
    pub fn new(x: i32, y: i32, beacon: Beacon) -> Self {
        Self::new_with_point(Point::new(x, y), beacon)
    }
    pub fn new_with_point(point: Point, beacon: Beacon) -> Self {
        Self {
            position: point,
            closest_beacon: beacon,
        }
    }
    pub fn ranges_with_impossible_beacons(&self) -> Vec<Point> {
        let distance_beacon = self.position.distance(self.closest_beacon.position);
        let mut points = vec![];
        let min_row = self.position.y() - distance_beacon;
        let max_row = self.position.y() + distance_beacon;
        let min_col = self.position.x() - distance_beacon;
        let max_col = self.position.x() + distance_beacon;

        // add top-right cuadrant
        for i in self.position.x()..=max_col {
            // we move i to be zero-based
            let adapted_i = i - self.position.x();
            for j in min_row + adapted_i..=self.position.y() {
                let point = Point::new(i, j);
                if point == self.position {
                    continue;
                }
                points.push(point);
            }
        }
        // add bottom-right corner
        for i in self.position.x()..=max_col {
            // we move i to be zero-based
            let adapted_i = i - self.position.x();
            for j in self.position.y() + 1..=max_row - adapted_i {
                let point = Point::new(i, j);
                if point == self.position {
                    continue;
                }
                points.push(point);
            }
        }

        // add bottom-left corner
        for i in min_col..self.position.x() {
            // we move i to be zero-based
            let adapted_i = i - min_col;
            for j in self.position.y()..=self.position.y() + adapted_i {
                let point = Point::new(i, j);
                if point == self.position {
                    continue;
                }
                points.push(point);
            }
        }

        // add top-left cuadrant
        for i in min_col + 1..self.position.x() {
            // we move i to be zero-based
            let adapted_i = i - min_col;
            for j in self.position.y() - adapted_i..self.position.y() {
                let point = Point::new(i, j);
                if point == self.position {
                    continue;
                }
                points.push(point);
            }
        }

        points
    }
}

impl PartialEq for MapPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
