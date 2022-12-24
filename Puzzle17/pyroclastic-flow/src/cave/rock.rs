use super::{BoundingBox, Point};
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
