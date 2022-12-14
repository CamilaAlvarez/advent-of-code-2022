use std::collections::HashMap;

pub type Item = u64;
#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Number(u64),
    OldValue,
}
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
}
#[derive(Debug, Clone)]
pub enum LazyOperation {
    Number(Item, HashMap<Item, Item>),
    Add(Box<LazyOperation>, Box<LazyOperation>, HashMap<Item, Item>),
    Multiply(Box<LazyOperation>, Box<LazyOperation>, HashMap<Item, Item>),
}

#[derive(Debug, Clone)]
pub struct BinaryOperation {
    operand1: Operand,
    operator: Operator,
    operand2: Operand,
}

impl LazyOperation {
    pub fn create_number(x: Item, modulo_options: &Vec<Item>) -> Box<Self> {
        let mut modulos = HashMap::new();
        for item in modulo_options.iter() {
            modulos.insert(*item, x % *item);
        }
        Box::new(Self::Number(x, modulos))
    }

    pub fn unwrap(&self) -> Item {
        match self {
            Self::Number(value, _) => *value,
            Self::Add(v1, v2, _) => v1.unwrap() + v2.unwrap(),
            Self::Multiply(v1, v2, _) => v1.unwrap() * v2.unwrap(),
        }
    }
    pub fn is_divisible_by(&self, x: Item) -> bool {
        match self {
            Self::Number(v, modulos) => {
                if let Some(modulo) = modulos.get(&x) {
                    return *modulo == 0;
                }
                *v % x == 0
            }
            Self::Add(_, _, modulos) => {
                if let Some(modulo) = modulos.get(&x) {
                    return *modulo == 0;
                }
                self.get_modulo(x) == 0
            }
            Self::Multiply(v1, v2, modulos) => {
                if let Some(modulo) = modulos.get(&x) {
                    return *modulo == 0;
                }
                v2.is_divisible_by(x) || v1.is_divisible_by(x)
            }
        }
    }
    fn get_modulo(&self, x: Item) -> Item {
        match self {
            Self::Number(v, _) => *v % x,
            Self::Add(v1, v2, _) => (v1.get_modulo(x) + v2.get_modulo(x)) % x,
            Self::Multiply(v1, v2, _) => (v1.get_modulo(x) * v2.get_modulo(x)) % x,
        }
    }
    pub fn get_computed_modulos(&self) -> &HashMap<Item, Item> {
        match self {
            Self::Number(_, modulos) => modulos,
            Self::Add(_, _, modulos) => modulos,
            Self::Multiply(_, _, modulos) => modulos,
        }
    }
}

impl BinaryOperation {
    pub fn new(operand1: String, operator: String, operand2: String) -> Self {
        let operand1 = Self::parse_operand(operand1);
        let operand2 = Self::parse_operand(operand2);
        let operator = Self::parse_operator(operator);
        Self {
            operand1,
            operator,
            operand2,
        }
    }
    pub fn execute(
        &self,
        x: &Box<LazyOperation>,
        modulo_options: &Vec<Item>,
    ) -> Box<LazyOperation> {
        let operand1 = self.get_operand_value(self.operand1, x, modulo_options);
        let operand2 = self.get_operand_value(self.operand2, x, modulo_options);
        match self.operator {
            Operator::Add => {
                let mut sum_modulo = HashMap::new();
                let operand1_modulos = operand1.get_computed_modulos();
                let operand2_modulos = operand2.get_computed_modulos();
                for key in operand1_modulos.keys() {
                    if let Some(operand1_modulo) = operand1_modulos.get(key) {
                        if let Some(operand2_modulo) = operand2_modulos.get(key) {
                            let modulo = (operand1_modulo + operand2_modulo) % *key;
                            sum_modulo.insert(*key, modulo);
                        }
                    }
                }
                Box::new(LazyOperation::Add(operand1, operand2, sum_modulo))
            }
            Operator::Multiply => {
                let mut multiply_modulo = HashMap::new();
                let operand1_modulos = operand1.get_computed_modulos();
                let operand2_modulos = operand2.get_computed_modulos();
                for key in operand1_modulos.keys() {
                    if let Some(operand1_modulo) = operand1_modulos.get(key) {
                        if let Some(operand2_modulo) = operand2_modulos.get(key) {
                            let modulo = (operand1_modulo * operand2_modulo) % *key;
                            multiply_modulo.insert(*key, modulo);
                        }
                    }
                }
                Box::new(LazyOperation::Multiply(operand1, operand2, multiply_modulo))
            }
        }
    }

    fn get_operand_value(
        &self,
        operand: Operand,
        x: &Box<LazyOperation>,
        modulo_options: &Vec<Item>,
    ) -> Box<LazyOperation> {
        match operand {
            Operand::OldValue => x.clone(),
            Operand::Number(v) => LazyOperation::create_number(v, modulo_options),
        }
    }
    fn parse_operand(operand: String) -> Operand {
        match operand.trim().as_ref() {
            "old" => Operand::OldValue,
            _ => Operand::Number(operand.trim().parse::<u64>().unwrap()),
        }
    }
    fn parse_operator(operator: String) -> Operator {
        match operator.trim().as_ref() {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => Operator::Add,
        }
    }
}
