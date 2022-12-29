use crate::point::{BoundingBox, Point};
#[derive(Debug, PartialEq, Eq)]
pub struct Rock {
    pieces: Vec<Point>,
    bounding_box: BoundingBox,
}
impl Rock {
    pub fn new(pieces: Vec<Point>, bounding_box: BoundingBox) -> Self {
        Self {
            pieces,
            bounding_box,
        }
    }
    pub fn move_by(&mut self, point: Point) {
        self.bounding_box.move_by(point)
    }
    pub fn pieces(&self) -> Vec<Point> {
        let mut pieces = vec![];
        for piece in self.pieces.iter() {
            pieces.push(*piece + self.bounding_box.top_left());
        }
        pieces
    }
    pub fn width(&self) -> usize {
        self.bounding_box.width()
    }
    pub fn height(&self) -> usize {
        self.bounding_box.height()
    }
    pub fn top(&self) -> i32 {
        self.bounding_box.top_left().y()
    }
    pub fn bottom(&self) -> i32 {
        self.bounding_box.bottom_right().y()
    }
    pub fn left(&self) -> i32 {
        self.bounding_box.top_left().x()
    }
    pub fn right(&self) -> i32 {
        self.bounding_box.bottom_right().x()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_rock() {
        let expected_pieces = vec![Point::new(1, 4), Point::new(1, 5), Point::new(1, 6)];
        let mut rock = Rock::new(
            vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)],
            BoundingBox::new(0, 1, 1, 3),
        );
        let move_by = Point::new(1, 3);
        rock.move_by(move_by);
        assert_eq!(
            expected_pieces,
            rock.pieces(),
            "Pieces were moved incorrectly"
        );
    }
}
