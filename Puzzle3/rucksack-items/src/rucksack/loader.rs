use super::{BadgeGroup, Rucksack};
use std::fs;
pub fn load_rucksack(filename: &str) -> Vec<Rucksack> {
    let mut rucksacks = vec![];
    let file_content = fs::read_to_string(filename).unwrap();
    for line in file_content.lines() {
        if let Some(rucksack) = Rucksack::new(line) {
            rucksacks.push(rucksack);
        }
    }
    rucksacks
}
pub fn load_badgegroup(filename: &str) -> Vec<BadgeGroup> {
    let mut badgegroups = vec![];
    let file_content = fs::read_to_string(filename).unwrap();
    let mut count = 0;
    let mut group = vec![];
    for line in file_content.lines() {
        if count < 3 {
            group.push(line.trim());
            count += 1;
        } else {
            if let Some(badgegroup) = BadgeGroup::new(group[0], group[1], group[2]) {
                badgegroups.push(badgegroup);
            }
            count = 1;
            group.clear();
            group.push(line.trim());
        }
    }
    if count >= 3 {
        if let Some(badgegroup) = BadgeGroup::new(group[0], group[1], group[2]) {
            badgegroups.push(badgegroup);
        }
    }
    badgegroups
}
