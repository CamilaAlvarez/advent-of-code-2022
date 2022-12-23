use std::{cell::RefCell, hash::Hash, rc::Rc};

use crate::valve::Valve;

#[derive(Clone)]
pub struct PositionAtValveKey {
    valve1_name: String,
    valve2_name: Option<String>,
}
impl PositionAtValveKey {
    pub fn new_single_key(valve_name: String) -> Self {
        Self {
            valve1_name: valve_name,
            valve2_name: None,
        }
    }
}
#[derive(PartialEq, Eq)]
pub enum PositionAtTime {
    Single(Rc<RefCell<Valve>>),
    Double(Rc<RefCell<Valve>>, Rc<RefCell<Valve>>),
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ValueAtTime {
    Single(i32),
    Double(i32, i32),
}
#[derive(PartialEq, Eq)]
pub struct State {
    pub time: i32,
    pub total_value: i32,
    pub value_per_minute: ValueAtTime,
    pub position: PositionAtTime,
    pub opened_valves: Vec<String>,
    pub steps: Vec<String>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other
            .time
            .cmp(&self.time)
            .then_with(|| self.total_value.cmp(&other.total_value))
            .then_with(|| match self.value_per_minute {
                ValueAtTime::Single(value) => match other.value_per_minute {
                    ValueAtTime::Single(v1) => {
                        return value.cmp(&v1);
                    }
                    ValueAtTime::Double(v1, v2) => {
                        return value.cmp(&(v1 + v2));
                    }
                },
                ValueAtTime::Double(value1, value2) => match other.value_per_minute {
                    ValueAtTime::Single(v1) => {
                        return (value1 + value2).cmp(&v1);
                    }
                    ValueAtTime::Double(v1, v2) => {
                        return (value1 + value2).cmp(&(v1 + v2));
                    }
                },
            });
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Hash for PositionAtValveKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.valve1_name.hash(state);
        if let Some(valve2_name) = &self.valve2_name {
            valve2_name.hash(state);
        }
    }
}
impl PartialEq for PositionAtValveKey {
    fn eq(&self, other: &Self) -> bool {
        match &self.valve2_name {
            Some(valve2) => {
                if let Some(other_valve2) = &other.valve2_name {
                    return (self.valve1_name == other.valve1_name && valve2 == other_valve2)
                        || (self.valve1_name == *other_valve2 && *valve2 == other.valve1_name);
                }
                return false;
            }
            None => {
                if other.valve2_name.is_some() {
                    return false;
                }
                return self.valve1_name == other.valve1_name;
            }
        }
    }
}
impl Eq for PositionAtValveKey {}
