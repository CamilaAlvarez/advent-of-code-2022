type TreeMap = Vec<Vec<u32>>;
type VisibilityMap = Vec<Vec<(u32, bool)>>;
pub struct TreeHeightMap {
    tree_map: TreeMap,
    top_max_height_map: VisibilityMap,
    bottom_max_height_map: VisibilityMap,
    left_max_height_map: VisibilityMap,
    right_max_height_map: VisibilityMap,
}

impl TreeHeightMap {
    pub fn new(tree_data: String) -> Self {
        let mut rows = vec![];
        for line in tree_data.lines() {
            let mut row = vec![];
            let heights = line.trim().chars();
            for height in heights {
                row.push(height.to_digit(10).unwrap());
            }
            rows.push(row);
        }
        Self {
            top_max_height_map: TreeHeightMap::build_top_max_height(&rows),
            bottom_max_height_map: TreeHeightMap::build_bottom_max_height(&rows),
            left_max_height_map: TreeHeightMap::build_left_max_height(&rows),
            right_max_height_map: TreeHeightMap::build_right_max_height(&rows),
            tree_map: rows,
        }
    }
    fn build_top_max_height(tree_map: &TreeMap) -> VisibilityMap {
        let mut top_map = vec![];
        for i in 0..tree_map.len() {
            // we just copy the first row
            if i == 0 {
                top_map.push(
                    tree_map[i]
                        .iter()
                        .map(|height| (*height, true))
                        .collect::<Vec<_>>(),
                );
                continue;
            }
            let mut new_row = vec![(0, false); tree_map[i].len()];
            for j in 0..tree_map[i].len() {
                if tree_map[i][j] > top_map[i - 1][j].0 {
                    new_row[j] = (tree_map[i][j], true);
                } else {
                    new_row[j] = (top_map[i - 1][j].0, false);
                }
            }
            top_map.push(new_row);
        }
        top_map
    }
    fn build_bottom_max_height(tree_map: &TreeMap) -> VisibilityMap {
        let mut bottom_map = vec![vec![]; tree_map.len()];
        for i in (0..tree_map.len()).rev() {
            // we just copy the first row
            if i == tree_map.len() - 1 {
                bottom_map[i] = tree_map[i]
                    .iter()
                    .map(|height| (*height, true))
                    .collect::<Vec<_>>();
                continue;
            }
            let mut new_row = vec![(0, true); tree_map[i].len()];
            for j in (0..tree_map[i].len()).rev() {
                if tree_map[i][j] > bottom_map[i + 1][j].0 {
                    new_row[j] = (tree_map[i][j], true);
                } else {
                    new_row[j] = (bottom_map[i + 1][j].0, false);
                }
            }
            bottom_map[i] = new_row;
        }
        bottom_map
    }
    fn build_left_max_height(tree_map: &TreeMap) -> VisibilityMap {
        let mut left_map = vec![];
        for i in 0..tree_map.len() {
            let mut new_row = vec![(0, true); tree_map[i].len()];
            for j in 0..tree_map[i].len() {
                // we just copy the first row
                if j == 0 {
                    new_row[j] = (tree_map[i][j], true);
                    continue;
                }

                if tree_map[i][j] > new_row[j - 1].0 {
                    new_row[j] = (tree_map[i][j], true);
                } else {
                    new_row[j] = (new_row[j - 1].0, false);
                }
            }
            left_map.push(new_row);
        }
        left_map
    }
    fn build_right_max_height(tree_map: &TreeMap) -> VisibilityMap {
        let mut right_map = Vec::with_capacity(tree_map.len());
        for i in 0..tree_map.len() {
            let mut new_row = vec![(0, true); tree_map[i].len()];
            for j in (0..tree_map[i].len()).rev() {
                // we just copy the first row
                if j == tree_map.len() - 1 {
                    new_row[j] = (tree_map[i][j], true);
                    continue;
                }

                if tree_map[i][j] > new_row[j + 1].0 {
                    new_row[j] = (tree_map[i][j], true);
                } else {
                    new_row[j] = (new_row[j + 1].0, false);
                }
            }
            right_map.push(new_row);
        }
        right_map
    }
    pub fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        assert!(x < self.tree_map.len(), "Invalid x position");
        assert!(x < self.tree_map[x].len(), "Invalid y position");
        // first row and col are alway visible
        if x == 0 || y == 0 {
            return true;
        }
        // last row and col are always visible
        else if x == self.tree_map.len() - 1 || y == self.tree_map[x].len() - 1 {
            return true;
        }
        let tree_height = self.tree_map[x][y];
        (tree_height >= self.top_max_height_map[x][y].0 && self.top_max_height_map[x][y].1)
            || (tree_height >= self.bottom_max_height_map[x][y].0
                && self.bottom_max_height_map[x][y].1)
            || (tree_height >= self.left_max_height_map[x][y].0 && self.left_max_height_map[x][y].1)
            || (tree_height >= self.right_max_height_map[x][y].0
                && self.right_max_height_map[x][y].1)
    }
    pub fn visible_tree_map(&self) -> Vec<Vec<bool>> {
        // Each max tree also needs to mention whether that specific spot held the maximum or not
        let mut visible_map = Vec::with_capacity(self.tree_map.len());
        for i in 0..self.tree_map.len() {
            let mut row = vec![];
            for j in 0..self.tree_map[i].len() {
                row.push(self.is_tree_visible(i, j));
            }
            visible_map.push(row);
        }
        visible_map
    }
    pub fn get_scores_map(&self) -> Vec<Vec<u32>> {
        let mut scores_map = Vec::with_capacity(self.tree_map.len());
        for i in 0..self.tree_map.len() {
            if i == 0 || i == self.tree_map.len() - 1 {
                scores_map.push(vec![0; self.tree_map[i].len()]);
                continue;
            }
            let mut row = vec![];
            for j in 0..self.tree_map[i].len() {
                if j == 0 || j == self.tree_map.len() - 1 {
                    row.push(0);
                } else {
                    row.push(self.get_score_for_point(i, j));
                }
            }
            scores_map.push(row);
        }
        scores_map
    }
    fn get_score_for_point(&self, x: usize, y: usize) -> u32 {
        let tree_height = self.tree_map[x][y];
        let top_score =
            if tree_height >= self.top_max_height_map[x][y].0 && self.top_max_height_map[x][y].1 {
                x as u32
            } else {
                let mut score = 0;
                for i in (0..x).rev() {
                    score += 1;
                    if self.top_max_height_map[i][y].1 || tree_height <= self.tree_map[i][y] {
                        break;
                    }
                }
                score
            };
        let bottom_score = if tree_height >= self.bottom_max_height_map[x][y].0
            && self.bottom_max_height_map[x][y].1
        {
            (self.tree_map.len() - 1 - x) as u32
        } else {
            let mut score = 0;
            for i in (x + 1)..self.tree_map.len() {
                score += 1;
                if self.bottom_max_height_map[i][y].1 || tree_height <= self.tree_map[i][y] {
                    break;
                }
            }
            score
        };
        let left_score = if tree_height >= self.left_max_height_map[x][y].0
            && self.left_max_height_map[x][y].1
        {
            y as u32
        } else {
            let mut score = 0;
            for j in (0..y).rev() {
                score += 1;
                if self.left_max_height_map[x][j].1 || tree_height <= self.tree_map[x][j] {
                    break;
                }
            }
            score
        };
        let right_score = if tree_height >= self.right_max_height_map[x][y].0
            && self.right_max_height_map[x][y].1
        {
            (self.tree_map[x].len() - 1 - y) as u32
        } else {
            let mut score = 0;
            for j in (y + 1)..self.tree_map[x].len() {
                score += 1;
                if self.right_max_height_map[x][j].1 || tree_height <= self.tree_map[x][j] {
                    break;
                }
            }
            score
        };
        top_score * bottom_score * left_score * right_score
    }
}
