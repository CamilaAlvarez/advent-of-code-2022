use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Valve {
    name: String,
    flow_rate: i32,
    is_opened: bool,
    can_be_opened: bool,
    neighbors: Vec<Weak<RefCell<Valve>>>,
}

impl Valve {
    // Rc<RefCell<Self>> a value that can have many owners and be mutably borrowed by any of them
    // IMPORTANT: These are not thread safe
    pub fn new(name: String, flow_rate: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name,
            flow_rate: flow_rate,
            is_opened: false,
            neighbors: vec![],
            can_be_opened: flow_rate > 0,
        }))
    }
    pub fn add_neighbor(&mut self, neighbor: &Rc<RefCell<Valve>>) {
        self.neighbors.push(Rc::downgrade(&neighbor));
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn open_valve(&mut self) {
        self.is_opened = true;
    }
    pub fn is_opened(&self) -> bool {
        self.is_opened
    }
    pub fn flow_rate(&self) -> i32 {
        self.flow_rate
    }
    pub fn neighbors(&self) -> &Vec<Weak<RefCell<Valve>>> {
        &self.neighbors
    }
    pub fn can_be_opened(&self) -> bool {
        self.can_be_opened
    }
    pub fn neighbors_remaining_flow(&self, opened_valves: &Vec<String>) -> i32 {
        let mut flow = 0;
        for neighbor in self.neighbors().iter() {
            if let Some(neighbor_ref) = Weak::upgrade(&neighbor) {
                let borrowed_neighbor = neighbor_ref.borrow();
                if !opened_valves.contains(&borrowed_neighbor.name())
                    && borrowed_neighbor.can_be_opened()
                {
                    flow += borrowed_neighbor.flow_rate();
                }
            }
        }
        flow
    }
}
impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.flow_rate == other.flow_rate
    }
}
impl Eq for Valve {}
