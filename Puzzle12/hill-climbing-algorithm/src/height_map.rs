pub struct HeightMap {
    height_map: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}
pub type NodeItem = u8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    x: usize,
    y: usize,
    value: NodeItem,
}
impl Node {
    pub fn new(x: usize, y: usize, value: NodeItem) -> Self {
        Self { x, y, value }
    }
    pub fn value(&self) -> NodeItem {
        self.value
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}
impl HeightMap {
    pub fn new_from_string(height_map_str: String) -> Self {
        let mut height_map_chars = vec![];
        for line in height_map_str.lines() {
            let chars = line.chars();
            let mut row = vec![];
            for c in chars {
                if c == 'S' {
                    row.push('a');
                } else if c == 'E' {
                    row.push('z');
                } else {
                    row.push(c);
                }
            }
            height_map_chars.push(row);
        }
        Self::new(&height_map_chars)
    }
    pub fn new(char_map: &Vec<Vec<char>>) -> Self {
        assert!(char_map.len() >= 1, "Empty height map");
        let height_map: Vec<Vec<u8>> = char_map
            .iter()
            .map(|row| row.iter().map(|c| *c as u8 - 'a' as u8).collect())
            .collect();
        Self {
            height_map,
            height: char_map.len(),
            width: char_map[0].len(),
        }
    }
    pub fn node(&self, x: usize, y: usize) -> Node {
        assert!(x < self.height, "Invalid x position");
        assert!(y < self.width, "Invalid y position");
        Node::new(x, y, self.height_map[x][y])
    }
    pub fn adjacent_nodes(&self, x: usize, y: usize) -> Vec<Node> {
        let mut adjecent_nodes = vec![];
        let node = self.node(x, y);
        self.add_vertical_adjacent(&node, &mut adjecent_nodes);
        self.add_horizontal_adjacent(&node, &mut adjecent_nodes);
        adjecent_nodes
    }

    fn add_vertical_adjacent(&self, node: &Node, adjecent_nodes: &mut Vec<Node>) {
        if node.x == 0 {
            adjecent_nodes.push(Node::new(
                node.x + 1,
                node.y,
                self.height_map[node.x + 1][node.y],
            ));
        } else if node.x == self.height - 1 {
            adjecent_nodes.push(Node::new(
                node.x - 1,
                node.y,
                self.height_map[node.x - 1][node.y],
            ));
        } else {
            adjecent_nodes.push(Node::new(
                node.x + 1,
                node.y,
                self.height_map[node.x + 1][node.y],
            ));
            adjecent_nodes.push(Node::new(
                node.x - 1,
                node.y,
                self.height_map[node.x - 1][node.y],
            ));
        }
    }
    fn add_horizontal_adjacent(&self, node: &Node, adjecent_nodes: &mut Vec<Node>) {
        if node.y == 0 {
            adjecent_nodes.push(Node::new(
                node.x,
                node.y + 1,
                self.height_map[node.x][node.y + 1],
            ));
        } else if node.y == self.width - 1 {
            adjecent_nodes.push(Node::new(
                node.x,
                node.y - 1,
                self.height_map[node.x][node.y - 1],
            ));
        } else {
            adjecent_nodes.push(Node::new(
                node.x,
                node.y + 1,
                self.height_map[node.x][node.y + 1],
            ));
            adjecent_nodes.push(Node::new(
                node.x,
                node.y - 1,
                self.height_map[node.x][node.y - 1],
            ));
        }
    }
    pub fn size(&self) -> usize {
        self.height * self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
}
