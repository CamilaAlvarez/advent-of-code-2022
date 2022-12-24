pub mod parser;
pub enum JetDirection {
    Left,
    Right,
}
impl JetDirection {
    pub fn numeric_value(&self) -> i32 {
        match self {
            Self::Left => 1,
            Self::Right => -1,
        }
    }
}
