use super::{JetDirection, JetPattern};

pub fn parse_jet_pattern(pattern_string: String) -> JetPattern {
    let mut directions = vec![];
    for char in pattern_string.chars() {
        if char == '>' {
            directions.push(JetDirection::Right);
        } else if char == '<' {
            directions.push(JetDirection::Left);
        }
    }
    JetPattern::new(directions)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_jet_pattern() {
        let jet_string = "<<>><".to_string();
        let expected_pattern = JetPattern::new(vec![
            JetDirection::Left,
            JetDirection::Left,
            JetDirection::Right,
            JetDirection::Right,
            JetDirection::Left,
        ]);
        assert_eq!(expected_pattern, parse_jet_pattern(jet_string));
    }
}
