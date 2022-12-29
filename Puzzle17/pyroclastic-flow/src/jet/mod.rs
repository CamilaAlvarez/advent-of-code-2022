use std::collections::VecDeque;

pub mod parser;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JetDirection {
    Left,
    Right,
}
#[derive(Debug, PartialEq, Eq)]
pub struct JetPattern {
    pattern: VecDeque<JetDirection>,
}
impl JetDirection {
    pub fn numeric_value(&self) -> i32 {
        match self {
            Self::Left => 1,
            Self::Right => -1,
        }
    }
}
impl JetPattern {
    pub fn new(pattern: Vec<JetDirection>) -> Self {
        Self {
            pattern: VecDeque::from(pattern),
        }
    }
}
impl Iterator for JetPattern {
    type Item = JetDirection;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.pattern.pop_front();
        if let Some(item) = next_item {
            self.pattern.push_back(item);
        }
        next_item
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_circular_iter() {
        let pattern = vec![JetDirection::Left, JetDirection::Left, JetDirection::Right];
        let mut jet_pattern = JetPattern::new(pattern.clone());

        assert_eq!(Some(JetDirection::Left), jet_pattern.next());
        assert_eq!(Some(JetDirection::Left), jet_pattern.next());
        assert_eq!(Some(JetDirection::Right), jet_pattern.next());
        assert_eq!(Some(JetDirection::Left), jet_pattern.next());
        assert_eq!(Some(JetDirection::Left), jet_pattern.next());
        assert_eq!(Some(JetDirection::Right), jet_pattern.next());
    }
}
