use pyroclastic_flow::{
    cave::{shape::RockShapeIterator, Cave},
    jet::parser::parse_jet_pattern,
};

#[test]
fn test_create_and_run_cave() {
    const NUM_ITERATIONS: usize = 2022;
    const EXPECTED_HEIGHT: i32 = 3068;
    let jet_string = String::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
    let jet_pattern = parse_jet_pattern(jet_string);
    let rock_iterator = RockShapeIterator::new();
    let mut cave = Cave::new(jet_pattern, rock_iterator);
    for _ in 0..NUM_ITERATIONS {
        cave.spawn_and_move_new_rock();
    }
    assert_eq!(EXPECTED_HEIGHT, cave.height());
}
