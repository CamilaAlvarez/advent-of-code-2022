use super::movement::MovementDirection;

pub type Position = (i32, i32);
pub struct RopePosition {
    knots: usize,
    knots_current_position: Vec<Position>,
    movement_map: Vec<Position>,
}

impl RopePosition {
    fn construct_with_position_knots(x: i32, y: i32, knots: usize) -> Self {
        Self {
            knots: knots,
            knots_current_position: vec![(x, y); knots],
            movement_map: vec![(x, y)],
        }
    }
    pub fn new_with_position(x: i32, y: i32) -> Self {
        Self::construct_with_position_knots(x, y, 2)
    }
    pub fn new() -> Self {
        RopePosition::new_with_position(0, 0)
    }
    pub fn new_with_knots(knots: usize) -> Self {
        Self::construct_with_position_knots(0, 0, knots)
    }
    pub fn move_rope(&mut self, direction: MovementDirection) {
        self.update_head_position(direction);
        for i in 1..self.knots {
            self.update_knot_position(i);
        }
        self.movement_map.push(self.get_tail_position());
    }
    pub fn get_tail_position(&self) -> Position {
        self.knots_current_position[self.knots - 1]
    }
    fn update_head_position(&mut self, direction: MovementDirection) {
        let head_position = self.knots_current_position[0];
        self.knots_current_position[0] = match direction {
            MovementDirection::Up => (head_position.0 - 1, head_position.1),
            MovementDirection::Down => (head_position.0 + 1, head_position.1),
            MovementDirection::Left => (head_position.0, head_position.1 - 1),
            MovementDirection::Right => (head_position.0, head_position.1 + 1),
        }
    }
    fn update_knot_position(&mut self, index: usize) {
        let current_knot_position = self.knots_current_position[index];
        let previous_knot_position = self.knots_current_position[index - 1];
        if previous_knot_position.0 == current_knot_position.0 {
            // head and tail share their row
            let horizontal_distance = previous_knot_position.1 - current_knot_position.1;
            if horizontal_distance.abs() >= 2 {
                // if the head and tail share position when won't enter this if
                if horizontal_distance < 0 {
                    self.knots_current_position[index] =
                        (current_knot_position.0, current_knot_position.1 - 1);
                } else {
                    self.knots_current_position[index] =
                        (current_knot_position.0, current_knot_position.1 + 1);
                }
            }
        } else if previous_knot_position.1 == current_knot_position.1 {
            // head and tail share their column
            let vertical_distance = previous_knot_position.0 - current_knot_position.0;
            if vertical_distance.abs() >= 2 {
                if vertical_distance < 0 {
                    self.knots_current_position[index] =
                        (current_knot_position.0 - 1, current_knot_position.1);
                } else {
                    self.knots_current_position[index] =
                        (current_knot_position.0 + 1, current_knot_position.1);
                }
            }
        } else {
            let horizontal_distance = previous_knot_position.1 - current_knot_position.1;
            let vertical_distance = previous_knot_position.0 - current_knot_position.0;
            if horizontal_distance.abs() >= 2 || vertical_distance.abs() >= 2 {
                self.knots_current_position[index] = (
                    current_knot_position.0 + vertical_distance.signum(),
                    current_knot_position.1 + horizontal_distance.signum(),
                );
            }
        }
    }
    pub fn movement_map(&self) -> &Vec<Position> {
        &self.movement_map
    }
}
