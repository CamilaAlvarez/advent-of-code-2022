pub mod rock;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    top_left: Point,
    width: usize,
    height: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}
impl BoundingBox {
    pub fn new_with_points(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self::new(x1, y1, x2 - x1, y2 - y1)
    }
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            top_left: Point::new(x, y),
            width: width,
            height: height,
        }
    }
    pub fn move_by(&mut self, point: Point) {
        self.top_left += point
    }
    pub fn top_left(&self) -> Point {
        self.top_left
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn bottom_right(&self) -> Point {
        self.top_left + Point::new(self.width, self.height)
    }
}
impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
