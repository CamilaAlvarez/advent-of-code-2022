use super::rock::Rock;
use crate::point::{BoundingBox, Point};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockShape {
    HorizontalLine,
    PlusSign,
    InvertedL,
    VerticalLine,
    Square,
}
pub struct RockShapeIterator {
    current_shape: RockShape,
}
impl RockShape {
    pub fn spawn_rock(&self, bottom_left: Point) -> Rock {
        match self {
            Self::HorizontalLine => {
                let points = vec![
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(2, 0),
                    Point::new(3, 0),
                ];
                let height = 1;
                let bbox = BoundingBox::new_with_top_left(
                    bottom_left + Point::new(0, height - 1),
                    4,
                    height as usize,
                );
                Rock::new(points, bbox)
            }
            Self::PlusSign => {
                let points = vec![
                    Point::new(1, 0),
                    Point::new(0, -1),
                    Point::new(1, -1),
                    Point::new(2, -1),
                    Point::new(1, -2),
                ];
                let height = 3;
                let bbox = BoundingBox::new_with_top_left(
                    bottom_left + Point::new(0, height - 1),
                    3,
                    height as usize,
                );
                Rock::new(points, bbox)
            }
            Self::InvertedL => {
                let points = vec![
                    Point::new(2, 0),
                    Point::new(2, -1),
                    Point::new(0, -2),
                    Point::new(1, -2),
                    Point::new(2, -2),
                ];
                let height = 3;
                let bbox = BoundingBox::new_with_top_left(
                    bottom_left + Point::new(0, height - 1),
                    3,
                    height as usize,
                );
                Rock::new(points, bbox)
            }
            Self::VerticalLine => {
                let points = vec![
                    Point::new(0, 0),
                    Point::new(0, -1),
                    Point::new(0, -2),
                    Point::new(0, -3),
                ];
                let height = 4;
                let bbox = BoundingBox::new_with_top_left(
                    bottom_left + Point::new(0, height - 1),
                    1,
                    height as usize,
                );
                Rock::new(points, bbox)
            }
            Self::Square => {
                let points = vec![
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(0, -1),
                    Point::new(1, -1),
                ];
                let height = 2;
                let bbox = BoundingBox::new_with_top_left(
                    bottom_left + Point::new(0, height - 1),
                    2,
                    height as usize,
                );
                Rock::new(points, bbox)
            }
        }
    }
}
impl RockShapeIterator {
    pub fn new() -> Self {
        Self {
            current_shape: RockShape::HorizontalLine,
        }
    }
}
impl Iterator for RockShapeIterator {
    type Item = RockShape;

    fn next(&mut self) -> Option<Self::Item> {
        let old_shape = self.current_shape;
        match self.current_shape {
            RockShape::HorizontalLine => {
                self.current_shape = RockShape::PlusSign;
            }
            RockShape::PlusSign => self.current_shape = RockShape::InvertedL,
            RockShape::InvertedL => self.current_shape = RockShape::VerticalLine,
            RockShape::VerticalLine => self.current_shape = RockShape::Square,
            RockShape::Square => self.current_shape = RockShape::HorizontalLine,
        }
        Some(old_shape)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_horizontal_line_rock() {
        let expected = Rock::new(
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ],
            BoundingBox::new_with_top_left(Point::new(0, 0), 4, 1),
        );
        let bottom_left = Point::new(0, 0);
        let result = RockShape::HorizontalLine.spawn_rock(bottom_left);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_create_plus_sign_rock() {
        let expected = Rock::new(
            vec![
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(2, -1),
                Point::new(1, -2),
            ],
            BoundingBox::new_with_top_left(Point::new(0, 2), 3, 3),
        );
        let bottom_left = Point::new(0, 0);
        let result = RockShape::PlusSign.spawn_rock(bottom_left);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_create_inverted_l_rock() {
        let expected = Rock::new(
            vec![
                Point::new(2, 0),
                Point::new(2, -1),
                Point::new(0, -2),
                Point::new(1, -2),
                Point::new(2, -2),
            ],
            BoundingBox::new_with_top_left(Point::new(0, 2), 3, 3),
        );
        let bottom_left = Point::new(0, 0);
        let result = RockShape::InvertedL.spawn_rock(bottom_left);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_create_vertical_line_rock() {
        let expected = Rock::new(
            vec![
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2),
                Point::new(0, -3),
            ],
            BoundingBox::new_with_top_left(Point::new(0, 3), 1, 4),
        );
        let bottom_left = Point::new(0, 0);
        let result = RockShape::VerticalLine.spawn_rock(bottom_left);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_create_square_rock() {
        let expected = Rock::new(
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(1, -1),
            ],
            BoundingBox::new_with_top_left(Point::new(0, 1), 2, 2),
        );
        let bottom_left = Point::new(0, 0);
        let result = RockShape::Square.spawn_rock(bottom_left);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_circular_shape_iterator() {
        let mut iterator = RockShapeIterator::new();
        assert_eq!(Some(RockShape::HorizontalLine), iterator.next());
        assert_eq!(Some(RockShape::PlusSign), iterator.next());
        assert_eq!(Some(RockShape::InvertedL), iterator.next());
        assert_eq!(Some(RockShape::VerticalLine), iterator.next());
        assert_eq!(Some(RockShape::Square), iterator.next());
        assert_eq!(Some(RockShape::HorizontalLine), iterator.next());
    }
}
