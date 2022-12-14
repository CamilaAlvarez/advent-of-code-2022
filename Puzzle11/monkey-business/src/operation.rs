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
    Number(HashMap<Item, Item>),
    Add(HashMap<Item, Item>),
    Multiply(HashMap<Item, Item>),
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
        Box::new(Self::Number(modulos))
    }

    pub fn is_divisible_by(&self, x: Item) -> bool {
        match self {
            Self::Number(modulos) => {
                if let Some(modulo) = modulos.get(&x) {
                    return *modulo == 0;
                }
                false
            }
            Self::Add(modulos) => {
                if let Some(modulo) = modulos.get(&x) {
                    return *modulo == 0;
                }
                false
            }
            Self::Multiply(modulos) => {
                if let Some(modulo) = modulos.get(&x) {
                    return *modulo == 0;
                }
                false
            }
        }
    }
    pub fn get_computed_modulos(&self) -> &HashMap<Item, Item> {
        match self {
            Self::Number(modulos) => modulos,
            Self::Add(modulos) => modulos,
            Self::Multiply(modulos) => modulos,
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
        // we don't care about the new values themselves, we only care about the modulos
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
                Box::new(LazyOperation::Add(sum_modulo))
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
                Box::new(LazyOperation::Multiply(multiply_modulo))
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
