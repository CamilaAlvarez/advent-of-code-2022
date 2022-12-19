use super::distance::Point;
use std::ops;

pub const MIN_COORDINATE_VALUE: i32 = 0;
pub const MAX_COORDINATE_VALUE: i32 = 4000000;

pub struct SensorsReachMap {
    sensors_ranges: Vec<ops::RangeInclusive<i32>>,
    occupied_spots: Vec<Point>,
    y: i32,
    locate_beacon: bool,
}

pub struct Beacon {
    position: Point,
}
pub struct Sensor {
    position: Point,
    closest_beacon: Beacon,
}

impl SensorsReachMap {
    pub fn new(y: i32, locate_beacon: bool) -> Self {
        Self {
            sensors_ranges: vec![],
            occupied_spots: vec![],
            y,
            locate_beacon,
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
        if let Some(range) = &sensor.horizontal_ranges_with_impossible_beacons(self.y) {
            let mut overlapped = false;
            for current_range in self.sensors_ranges.iter_mut() {
                if range.start() <= current_range.end() && current_range.start() <= range.end() {
                    *current_range = Self::merge_ranges(current_range, range);
                    overlapped = true;
                    break;
                }
            }
            if overlapped {
                // If we overlapped we might be missing some overlap between old ranges and the new one
                let mut new_ranges = vec![];
                self.sensors_ranges
                    .sort_by(|item1, item2| item1.start().cmp(&item2.start()));
                for current_range in self.sensors_ranges.iter() {
                    if new_ranges.is_empty() {
                        new_ranges.push(current_range.clone());
                    } else {
                        if let Some(last_interval) = new_ranges.pop() {
                            if last_interval.start() <= current_range.end()
                                && current_range.start() <= last_interval.end()
                            {
                                new_ranges.push(Self::merge_ranges(current_range, &last_interval));
                            } else {
                                new_ranges.push(last_interval);
                                new_ranges.push(current_range.clone());
                            }
                        }
                    }
                }
                self.sensors_ranges = new_ranges;
            } else {
                self.sensors_ranges.push(range.clone());
            }
        }
    }
    pub fn number_no_possible_beacon_location(&self) -> Option<usize> {
        let mut occupied_points_in_range = 0;
        let mut elements = 0;
        for range in self.sensors_ranges.iter() {
            for point in self.occupied_spots.iter() {
                if point.y() == self.y && range.contains(&point.x()) {
                    occupied_points_in_range += 1;
                }
            }
            let valid_range = range.clone();
            if self.locate_beacon {
                if valid_range.start() <= &MIN_COORDINATE_VALUE
                    && valid_range.end() >= &MAX_COORDINATE_VALUE
                {
                    return Some((MAX_COORDINATE_VALUE - MIN_COORDINATE_VALUE) as usize);
                }
            }
            let elements_in_range = valid_range.end() - valid_range.start() + 1;
            elements += elements_in_range as usize - occupied_points_in_range;
        }
        Some(elements)
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
        let max_row = self.position.y() + distance_beacon;

        // if this row doesn't have a range for this beacon
        if y < min_row || y > max_row {
            return None;
        }
        if y < self.position.y() {
            let row = y - min_row;
            return Some(ops::RangeInclusive::new(
                self.position.x() - row,
                self.position.x() + row,
            ));
        } else {
            let row = y - self.position.y();
            return Some(ops::RangeInclusive::new(
                self.position.x() - distance_beacon + row,
                self.position.x() + distance_beacon - row,
            ));
        }
    }
}
