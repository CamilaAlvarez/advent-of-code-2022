use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}
// TODO: distance could be implemented as a trait
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, other: Point) -> i32 {
        self.horizontal_distance(other) + self.vertical_distance(other)
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn vertical_distance(&self, other: Point) -> i32 {
        (self.y - other.y).abs()
    }
    pub fn horizontal_distance(&self, other: Point) -> i32 {
        (self.x - other.x).abs()
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
