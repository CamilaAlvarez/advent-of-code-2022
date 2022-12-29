use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    top_left: Point,
    width: usize,
    height: usize,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}
impl BoundingBox {
    pub fn new_with_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        assert!(x2 > x1 && y2 > y1, "Invalid rectangle");
        Self::new(x1, y1, (x2 - x1) as usize, (y2 - y1) as usize)
    }
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        Self {
            top_left: Point::new(x, y),
            width: width,
            height: height,
        }
    }
    pub fn new_with_top_left(top_left: Point, width: usize, height: usize) -> Self {
        Self {
            top_left,
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
        self.top_left + Point::new(self.width as i32 - 1, -1 * self.height as i32 + 1)
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
impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_x = if self.x >= rhs.x { self.x - rhs.x } else { 0 };
        let new_y = if self.y >= rhs.y { self.y - rhs.y } else { 0 };
        Point::new(new_x, new_y)
    }
}
impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
