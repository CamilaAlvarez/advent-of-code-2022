use super::height_map::HeightMap;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CurrentState {
    x: usize,
    y: usize,
    cost: usize,
}

pub fn get_path_length(
    height_map: &HeightMap,
    source: (usize, usize),
    dest: (usize, usize),
) -> Option<usize> {
    let mut dists = (0..height_map.size())
        .map(|_| usize::MAX)
        .collect::<Vec<_>>();
    let mut min_heap = BinaryHeap::new();
    assert!(
        source.0 * height_map.width() + source.1 < height_map.size(),
        "Invalid source position"
    );
    assert!(
        dest.0 * height_map.width() + dest.1 < height_map.size(),
        "Invalid destination"
    );
    dists[source.0 * source.1] = 0;
    min_heap.push(CurrentState::new(source.0, source.1, 0));

    while let Some(CurrentState { x, y, cost }) = min_heap.pop() {
        if (x, y) == dest {
            return Some(cost);
        }
        if cost > dists[x * height_map.width() + y] {
            continue;
        }
        let current_node = height_map.node(x, y);
        for node in height_map.adjacent_nodes(x, y).iter() {
            let index = node.x() * height_map.width() + node.y();
            if node.value() > current_node.value() && node.value() - current_node.value() > 1 {
                continue;
            }
            let next = CurrentState::new(node.x(), node.y(), cost + 1);
            if next.cost < dists[index] {
                min_heap.push(next);
                dists[index] = next.cost;
            }
        }
    }
    None
}
impl CurrentState {
    pub fn new(x: usize, y: usize, cost: usize) -> Self {
        Self { x, y, cost }
    }
}
// To be able to use the node in a min heap
impl Ord for CurrentState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| (self.x * self.y).cmp(&(other.x * other.y)))
    }
}
impl PartialOrd for CurrentState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
