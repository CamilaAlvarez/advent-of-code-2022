pub type Point = (usize, usize);
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
    pub fn points_in_line(&self) -> Vec<Point> {
        let mut points = vec![];
        points.push(self.start);
        let (mut x1, mut y1) = self.start;
        let (x2, y2) = self.end;
        if x1 == x2 {
            while y1 != y2 {
                if y1 < y2 {
                    y1 += 1;
                } else {
                    y1 -= 1;
                }
                points.push((x1, y1));
            }
        } else if y1 == y2 {
            while x1 != x2 {
                if x1 < x2 {
                    x1 += 1;
                } else {
                    x1 -= 1;
                }
                points.push((x1, y1));
            }
        }
        points
    }

    pub fn parse_lines_from_string(lines_string: String) -> Vec<Self> {
        let mut lines = vec![];
        let points: Vec<&str> = lines_string.trim().split("->").collect();
        for i in 0..points.len() - 1 {
            if let Some(start_str) = points.get(i) {
                if let Some(end_str) = points.get(i + 1) {
                    if let Some(start) = parse_point(&start_str) {
                        if let Some(end) = parse_point(&end_str) {
                            lines.push(Self::new(start, end));
                        }
                    }
                }
            }
        }
        lines
    }
}

fn parse_point(point_str: &str) -> Option<Point> {
    let split_point: Vec<&str> = point_str.trim().split(",").collect();
    assert!(split_point.len() == 2, "Invalid point");
    if let Some(x_str) = split_point.get(0) {
        if let Some(y_str) = split_point.get(1) {
            if let Ok(x) = x_str.parse::<usize>() {
                if let Ok(y) = y_str.parse::<usize>() {
                    return Some((x, y));
                }
            }
        }
    }
    None
}
