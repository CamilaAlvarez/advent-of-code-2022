use crate::{jet::JetDirection, jet::JetPattern, point::Point};

use self::shape::RockShapeIterator;
use std::collections::HashMap;

pub mod rock;
pub mod shape;

const CAVE_WIDTH: i32 = 7;
const MAX_CAVE_INDEX: i32 = CAVE_WIDTH - 1;
const X_ORIGIN: i32 = 2;
const Y_FROM_TOP: i32 = 3;

pub struct Cave {
    filled_points_per_row: HashMap<i32, Vec<Point>>,
    shape_iterator: RockShapeIterator,
    jet_pattern: JetPattern,
}
impl Cave {
    pub fn new(jet_pattern: JetPattern, rock_iterator: RockShapeIterator) -> Self {
        Self {
            filled_points_per_row: HashMap::new(),
            shape_iterator: rock_iterator,
            jet_pattern,
        }
    }
    pub fn height(&self) -> i32 {
        // The height is the maximum key
        if let Some(max_key) = self.filled_points_per_row.keys().max() {
            return max_key + 1;
        }
        0
    }
    pub fn spawn_and_move_new_rock(&mut self) -> usize {
        let next_shape = self.shape_iterator.next();
        let mut number_jet_iterations = 0;
        if let Some(shape) = next_shape {
            let origin_y = self.height() + Y_FROM_TOP;
            let mut rock = shape.spawn_rock(Point::new(X_ORIGIN as i32, origin_y));
            // This is the minimum movement the rock will have Before colliding with anything.
            // At this point we should not collide with anything (because we start 3 points above the top-most point)
            let mut x_movement: i32 = 0;
            let y_movement: i32 = -Y_FROM_TOP;
            for _ in 0..Y_FROM_TOP {
                number_jet_iterations += 1;
                match self.jet_pattern.next() {
                    Some(JetDirection::Left) if rock.left() + x_movement > 0 => x_movement -= 1,
                    Some(JetDirection::Right) if rock.right() + x_movement < MAX_CAVE_INDEX => {
                        x_movement += 1
                    }
                    _ => {}
                }
            }
            rock.move_by(Point::new(x_movement, y_movement));
            // Now we'll start verifying collisions
            'movement: loop {
                // 1. Move the rock according to the jet pattern
                match self.jet_pattern.next() {
                    Some(JetDirection::Left) if rock.left() > 0 => {
                        let mut expected_movement = -1;
                        for point in rock.pieces() {
                            if let Some(points) = self.filled_points_per_row.get(&point.y()) {
                                let next_position = Point::new(point.x() - 1, point.y());
                                if points.contains(&next_position) {
                                    // If there is a collision we do not move
                                    expected_movement = 0;
                                    break;
                                }
                            }
                        }
                        rock.move_by(Point::new(expected_movement, 0));
                    }
                    Some(JetDirection::Right)
                        if rock.left() >= 0 && rock.right() < MAX_CAVE_INDEX =>
                    {
                        let mut expected_movement = 1;
                        for point in rock.pieces() {
                            if let Some(points) = self.filled_points_per_row.get(&point.y()) {
                                let next_position = Point::new(point.x() + 1, point.y());
                                if points.contains(&next_position) {
                                    // If there is a collision we do not move
                                    expected_movement = 0;
                                    break;
                                }
                            }
                        }
                        rock.move_by(Point::new(expected_movement, 0));
                    }
                    _ => {}
                }
                number_jet_iterations += 1;
                // 2. check if the next downwards position of the rock collides with another rock, or if it reached the ground
                let rock_bottom = rock.bottom();
                if rock_bottom <= 0 {
                    break;
                }
                // Since we ignore horizontal collisions, we only need to check if we're going to collide when moving down.
                // We need to check each row
                for row in rock_bottom..=rock.top() {
                    if let Some(points) = self.filled_points_per_row.get(&(row - 1)) {
                        for point in rock.pieces() {
                            let next_point = point - Point::new(0, 1);
                            if points.contains(&next_point) {
                                break 'movement;
                            }
                        }
                    }
                }

                // 3. Move the rock downwards
                rock.move_by(Point::new(0, -1));
            }
            for point in rock.pieces() {
                if let Some(points) = self.filled_points_per_row.get_mut(&point.y()) {
                    points.push(point);
                } else {
                    self.filled_points_per_row.insert(point.y(), vec![point]);
                }
            }
        }
        number_jet_iterations
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one_rock() {
        let mut cave = Cave::new(
            JetPattern::new(vec![JetDirection::Left, JetDirection::Right]),
            RockShapeIterator::new(),
        );
        let expected_height = 1;
        cave.spawn_and_move_new_rock();
        assert_eq!(expected_height, cave.height());
    }
    #[test]
    fn test_two_rock() {
        let mut cave = Cave::new(
            JetPattern::new(vec![
                JetDirection::Right,
                JetDirection::Right,
                JetDirection::Right,
                JetDirection::Left,
                JetDirection::Left,
                JetDirection::Right,
                JetDirection::Left,
                JetDirection::Right,
            ]),
            RockShapeIterator::new(),
        );
        let expected_height = 4;
        cave.spawn_and_move_new_rock();
        cave.spawn_and_move_new_rock();
        assert_eq!(expected_height, cave.height());
    }
}
