use std::ops;

use super::distance::Point;

pub struct SensorsReachMap {
    sensors_ranges: Option<ops::RangeInclusive<i32>>,
    occupied_spots: Vec<Point>,
    y: i32,
}

pub struct Beacon {
    position: Point,
}
pub struct Sensor {
    position: Point,
    closest_beacon: Beacon,
}

impl SensorsReachMap {
    pub fn new(y: i32) -> Self {
        Self {
            sensors_ranges: None,
            occupied_spots: vec![],
            y,
        }
    }
    pub fn add_sensor(&mut self, sensor: &Sensor) {
        if !self.occupied_spots.contains(&sensor.position) && sensor.position.y() == self.y {
            self.occupied_spots.push(sensor.position);
        }
        if !self
            .occupied_spots
            .contains(&sensor.closest_beacon.position)
            && sensor.closest_beacon.position.y() == self.y
        {
            self.occupied_spots.push(sensor.closest_beacon.position);
        }
        if let Some(range) = sensor.horizontal_ranges_with_impossible_beacons(self.y) {
            if let Some(sensors_range) = &self.sensors_ranges {
                self.sensors_ranges = Some(Self::merge_ranges(&sensors_range, &range));
            } else {
                self.sensors_ranges = Some(range);
            }
        }
    }
    pub fn number_no_possible_beacon_location(&self) -> Option<usize> {
        let mut occupied_points_in_range = 0;
        if let Some(range) = &self.sensors_ranges {
            for point in self.occupied_spots.iter() {
                if point.y() == self.y && range.contains(&point.x()) {
                    occupied_points_in_range += 1;
                }
            }
            let elements_in_range = range.clone().collect::<Vec<_>>().len();
            return Some(elements_in_range - occupied_points_in_range);
        }
        None
    }

    fn merge_ranges(
        current_range: &ops::RangeInclusive<i32>,
        other: &ops::RangeInclusive<i32>,
    ) -> ops::RangeInclusive<i32> {
        let mut start = current_range.start();
        let mut end = current_range.end();
        if other.start() < start {
            start = other.start();
        }
        if other.end() > end {
            end = other.end();
        }
        ops::RangeInclusive::new(*start, *end)
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
    pub fn horizontal_ranges_with_impossible_beacons(
        &self,
        y: i32,
    ) -> Option<ops::RangeInclusive<i32>> {
        let distance_beacon = self.position.distance(self.closest_beacon.position);
        let min_row = self.position.y() - distance_beacon;

        for row in 0..=distance_beacon {
            if min_row + row != y {
                continue;
            }
            return Some(ops::RangeInclusive::new(
                self.position.x() - row,
                self.position.x() + row,
            ));
        }
        for row in 1..=distance_beacon {
            if self.position.y() + row != y {
                continue;
            }
            return Some(ops::RangeInclusive::new(
                self.position.x() - distance_beacon + row,
                self.position.x() + distance_beacon - row,
            ));
        }
        None
    }
}
