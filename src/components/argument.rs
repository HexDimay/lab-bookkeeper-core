/// Является шаблоном полей ввода для структуры `Qualifier`.
#[derive(Debug, Clone)]
pub struct Argument {
    name: String,
    type_value: TypeValue,
}

impl Argument {
    pub fn new(name: String, type_value: TypeValue) -> Self {
        Self { name, type_value }
    }
}

impl Arg for Argument {
    fn name(&self) -> &String {
        &self.name
    }

    fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }

    fn type_value(&self) -> &TypeValue {
        &self.type_value
    }

    fn get_mut_type_value(&mut self) -> &mut TypeValue {
        &mut self.type_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeValue {
    Float,
    Int,
    String,
    Bool,
    Date,
}

pub trait Arg {
    fn name(&self) -> &String;
    fn get_mut_name(&mut self) -> &mut String;
    fn type_value(&self) -> &TypeValue;
    fn get_mut_type_value(&mut self) -> &mut TypeValue;
}
