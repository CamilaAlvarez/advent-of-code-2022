use super::path::{Line, Point};

const FLOOR_HEIGHT: usize = 2;
const INFINITE_FLOOR_FACTOR: usize = 3;

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
    sand_origin: Point,
    static_sand_units: usize,
    infinite_floor: bool,
    real_map_width: usize,
}

impl Cave {
    pub fn new(
        top_left: Point,
        sand_origin: Point,
        bottom_right: Point,
        lines: &Vec<Vec<Line>>,
        infinite_floor: bool,
    ) -> Self {
        let height =
            bottom_right.1 - top_left.1 + 1 + if infinite_floor { FLOOR_HEIGHT } else { 0 };
        let width = bottom_right.0 - top_left.0 + 1;
        let real_width = if infinite_floor {
            width * INFINITE_FLOOR_FACTOR
        } else {
            width
        };
        let local_sand_origin = (
            Self::get_horizontal_position(sand_origin, top_left, real_width, infinite_floor),
            sand_origin.1 - top_left.1,
        );
        // we create a bigger tile map to handle overflows
        // If it overflows after that we just need to increase its size
        let tiles = Self::create_tiles(
            height,
            width,
            real_width,
            lines,
            top_left,
            sand_origin,
            infinite_floor,
        );

        Self {
            height,
            sand_origin: local_sand_origin,
            tiles,
            static_sand_units: 0,
            infinite_floor,
            real_map_width: real_width,
        }
    }
    fn get_horizontal_position(
        point: Point,
        top_left_position: Point,
        real_width: usize,
        infinite_floor: bool,
    ) -> usize {
        point.0
            + if infinite_floor {
                real_width / INFINITE_FLOOR_FACTOR
            } else {
                0
            }
            - top_left_position.0
    }
    fn create_tiles(
        height: usize,
        width: usize,
        real_width: usize,
        lines: &Vec<Vec<Line>>,
        top_left: (usize, usize),
        sand_origin: (usize, usize),
        infinite_floor: bool,
    ) -> Vec<Vec<Tile>> {
        let mut tiles: Vec<Vec<Tile>> = vec![];
        for row_index in 0..height {
            let mut row = vec![];
            if infinite_floor && row_index == height - 1 {
                row = vec![Tile::Rock; INFINITE_FLOOR_FACTOR * width];
            } else {
                for _ in 0..if infinite_floor {
                    INFINITE_FLOOR_FACTOR * width
                } else {
                    width
                } {
                    row.push(Tile::Air)
                }
            }
            tiles.push(row)
        }
        // TODO: write in a safer way (using get instead of indexing)
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let line = &lines[i][j];
                for point in line.points_in_line().iter() {
                    let x_in_tiles =
                        Self::get_horizontal_position(*point, top_left, real_width, infinite_floor);
                    let y_in_tiles = point.1 - top_left.1;
                    tiles[y_in_tiles][x_in_tiles] = Tile::Rock;
                }
            }
        }
        let x_source =
            Self::get_horizontal_position(sand_origin, top_left, real_width, infinite_floor);
        let y_source = sand_origin.1 - top_left.1;
        tiles[y_source][x_source] = Tile::SandSource;
        tiles
    }

    fn set_tile_content(&mut self, point: Point, tile: Tile) {
        if point.0 < self.real_map_width && point.1 < self.height {
            self.tiles[point.1][point.0] = tile;
        }
    }

    fn tile_content(&self, point: Point) -> Option<Tile> {
        if point.0 < self.real_map_width && point.1 < self.height {
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
                    if falling_sand_position == self.sand_origin {
                        break 'sand_generation;
                    }
                } else {
                    // The sand could not move from its old position
                    // 1. Update tile map
                    if self.tile_content(falling_sand_position).is_some() {
                        self.set_tile_content(falling_sand_position, Tile::Sand);
                    }
                    // 2. Update number of static sand units
                    self.static_sand_units += 1;
                    // 3. Are we on the source itself?
                    if falling_sand_position == self.sand_origin {
                        break 'sand_generation;
                    }
                    // 4. Get a new sand particle down
                    break 'sand_movement;
                }
            }
        }
    }

    fn update_tile_map_size_for_inifinite_floor(&mut self) {
        let mut tiles: Vec<Vec<Tile>> = vec![];
        let top_left = (0, 0);
        self.real_map_width = self.real_map_width * INFINITE_FLOOR_FACTOR;
        for row_index in 0..self.height {
            let mut row = vec![];
            if row_index == self.height - 1 {
                row = vec![Tile::Rock; INFINITE_FLOOR_FACTOR * self.real_map_width];
            } else {
                for _ in 0..self.real_map_width {
                    row.push(Tile::Air)
                }
            }
            tiles.push(row)
        }
        // TODO: write in a safer way (using get instead of indexing)
        for j in 0..self.tiles.len() {
            for i in 0..self.tiles[j].len() {
                let point = (i, j);
                let x_in_tiles = Self::get_horizontal_position(
                    point,
                    top_left,
                    self.real_map_width,
                    self.infinite_floor,
                );
                tiles[point.1][x_in_tiles] = self.tiles[j][i];
            }
        }

        let x_source = Self::get_horizontal_position(
            self.sand_origin,
            top_left,
            self.real_map_width,
            self.infinite_floor,
        );
        tiles[self.sand_origin.1][x_source] = Tile::SandSource;
        self.sand_origin = (x_source, self.sand_origin.1);
        self.tiles = tiles;
    }

    fn get_next_sand_position(&mut self, falling_sand_position: Point) -> Option<Position> {
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
        if falling_sand_position.1 == self.height - 1 {
            // we're already at the edge!
            return Some(Position::OutOfBounds);
        }
        // we now have an infinitely long floor, if we are reaching outside of boundaries in x we need to check some special locations

        if falling_sand_position.0 == 0 && self.infinite_floor {
            // we need to increse the size of our map and transform sand origin
            self.update_tile_map_size_for_inifinite_floor();
        }
        let down_left_pos = (falling_sand_position.0 - 1, falling_sand_position.1 + 1);
        if self.tile_content(down_left_pos) == Some(Tile::Air)
            || self.tile_content(down_left_pos) == Some(Tile::SandSource)
        {
            return Some(Position::Point(down_left_pos));
        }

        // 3. Check if down-right is free
        // Check boundaries first!
        if falling_sand_position.1 == self.height - 1 {
            // we're already at the edge!
            return Some(Position::OutOfBounds);
        }
        // we now have an infinitely long floor, if we are reaching outside of boundaries in x we need to check some special locations
        if falling_sand_position.0 == self.real_map_width - 1 {
            // we need to increse the size of our map and transform sand origin
            self.update_tile_map_size_for_inifinite_floor();
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
