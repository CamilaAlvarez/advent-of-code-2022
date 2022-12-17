use super::path::{Line, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    OutOfBounds,
    Point(Point),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Air,
    Sand,
    Rock,
    SandSource,
}
pub struct Cave {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
    sand_origin: Point,
    static_sand_units: usize,
}

impl Cave {
    pub fn new(
        top_left: Point,
        sand_origin: Point,
        bottom_right: Point,
        lines: &Vec<Vec<Line>>,
    ) -> Self {
        let height = bottom_right.1 - top_left.1 + 1;
        let width = bottom_right.0 - top_left.0 + 1;
        let local_sand_origin = (sand_origin.0 - top_left.0, sand_origin.1 - top_left.1);
        let tiles = Self::create_tiles(height, width, lines, top_left, sand_origin);

        Self {
            height,
            width,
            sand_origin: local_sand_origin,
            tiles,
            static_sand_units: 0,
        }
    }
    fn create_tiles(
        height: usize,
        width: usize,
        lines: &Vec<Vec<Line>>,
        top_left: (usize, usize),
        sand_origin: (usize, usize),
    ) -> Vec<Vec<Tile>> {
        let mut tiles: Vec<Vec<Tile>> = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Tile::Air)
            }
            tiles.push(row)
        }
        // TODO: write in a safer way (using get instead of indexing)
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let line = &lines[i][j];
                for point in line.points_in_line().iter() {
                    let x_in_tiles = point.0 - top_left.0;
                    let y_in_tiles = point.1 - top_left.1;
                    tiles[y_in_tiles][x_in_tiles] = Tile::Rock;
                }
            }
        }
        let x_source = sand_origin.0 - top_left.0;
        let y_source = sand_origin.1 - top_left.1;
        tiles[y_source][x_source] = Tile::SandSource;
        tiles
    }

    fn set_tile_content(&mut self, point: Point, tile: Tile) {
        if point.0 < self.width && point.1 < self.height {
            self.tiles[point.1][point.0] = tile;
        }
    }

    fn tile_content(&self, point: Point) -> Option<Tile> {
        if point.0 < self.width && point.1 < self.height {
            return Some(self.tiles[point.1][point.0]);
        }
        None
    }

    pub fn flood_sand(&mut self) {
        'sand_generation: loop {
            let mut falling_sand_position = self.sand_origin;
            'sand_movement: loop {
                // The sand particle can keep moving
                if let Some(new_position) = self.get_next_sand_position(falling_sand_position) {
                    // Check if goes out of bounds
                    match new_position {
                        Position::Point(point) => falling_sand_position = point,
                        Position::OutOfBounds => break 'sand_generation,
                    }
                } else {
                    // The sand could not move from its old position
                    // 1. Update tile map
                    if self.tiles.get(falling_sand_position.0).is_some()
                        && self.tiles.get(falling_sand_position.1).is_some()
                    {
                        self.set_tile_content(falling_sand_position, Tile::Sand);
                    }
                    // 2. Update number of static sand units
                    self.static_sand_units += 1;
                    // 3. Get a new sand particle down
                    break 'sand_movement;
                }
            }
        }
    }

    fn get_next_sand_position(&self, falling_sand_position: Point) -> Option<Position> {
        // 1. Check if down is free
        // Check boundaries first!
        if falling_sand_position.1 == self.height - 1 {
            // we're already at the edge!
            return Some(Position::OutOfBounds);
        }
        let down_pos = (falling_sand_position.0, falling_sand_position.1 + 1);
        if self.tile_content(down_pos) == Some(Tile::Air)
            || self.tile_content(down_pos) == Some(Tile::SandSource)
        {
            return Some(Position::Point(down_pos));
        }

        // 2. Check if down-left is free
        // Check boundaries first!
        if falling_sand_position.1 == self.height - 1 || falling_sand_position.0 == 0 {
            // we're already at the edge!
            return Some(Position::OutOfBounds);
        }
        let down_left_pos = (falling_sand_position.0 - 1, falling_sand_position.1 + 1);
        if self.tile_content(down_left_pos) == Some(Tile::Air)
            || self.tile_content(down_left_pos) == Some(Tile::SandSource)
        {
            return Some(Position::Point(down_left_pos));
        }

        // 3. Check if down-right is free
        // Check boundaries first!
        if falling_sand_position.1 == self.height - 1 || falling_sand_position.0 == self.width - 1 {
            // we're already at the edge!
            return Some(Position::OutOfBounds);
        }
        let down_right_pos = (falling_sand_position.0 + 1, falling_sand_position.1 + 1);
        if self.tile_content(down_right_pos) == Some(Tile::Air)
            || self.tile_content(down_right_pos) == Some(Tile::SandSource)
        {
            return Some(Position::Point(down_right_pos));
        }

        // 4. There is no free space for the particle to move
        None
    }

    pub fn sand_units_at_rest(&self) -> usize {
        self.static_sand_units
    }
}
