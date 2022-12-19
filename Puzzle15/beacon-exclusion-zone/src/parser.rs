use super::distance::Point;
use super::sensor::{Beacon, Sensor, SensorsReachMap};
use regex::Regex;

pub fn parse_map(file_content: String) -> SensorsReachMap {
    let mut sensors = vec![];
    // TODO: handle error case
    let re = Regex::new(
        r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();
    for cap in re.captures_iter(&file_content) {
        let x_sensor = &cap[1].parse::<i32>().unwrap();
        let y_sensor = &cap[2].parse::<i32>().unwrap();
        let x_beacon = &cap[3].parse::<i32>().unwrap();
        let y_beacon = &cap[4].parse::<i32>().unwrap();
        let sensor_point = Point::new(*x_sensor, *y_sensor);
        let beacon_point = Point::new(*x_beacon, *y_beacon);
        let beacon = Beacon::new_with_point(beacon_point);
        let sensor = Sensor::new_with_point(sensor_point, beacon);
        sensors.push(sensor);
    }

    SensorsReachMap::new(&sensors)
}
