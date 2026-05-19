use crate::components::{argument::Arg, qualifier::Qualifier};

/// `Entity` - основная сущность над которой происходят операции.
pub struct Entity<A: Arg> {
    /// Не может быть изменён.
    id: uuid::Uuid,
    qualifier: Qualifier<A>,
    /// Позиция значения должна совпадать с позицией аргумента в определителе.
    values: Vec<Value>
}

impl<A: Arg> Entity<A> {
    pub fn new(qualifier: Qualifier<A>) -> Self {
        let values = Self::init_args(&qualifier);
        Self {
            id: uuid::Uuid::new_v4(),
            qualifier,
            values
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    /// Инициализация значений относительно аргументов.
    fn init_args(qualifier: &Qualifier<A>) -> Vec<Value> {
        qualifier.arguments().iter()
            .map(|a| {
                match a.type_value() {
                    super::argument::TypeValue::Float => Value::Float(Default::default()),
                    super::argument::TypeValue::Int => Value::Int(Default::default()),
                    super::argument::TypeValue::String => Value::String(Default::default()),
                    super::argument::TypeValue::Bool => Value::Bool(Default::default()),
                }
            })
            .collect()
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }

    pub fn get_mut_values(&mut self) -> &mut [Value] {
        &mut self.values
    }

    pub fn get_value_by_name(&self, name: &String) -> Option<&Value> {
        self.values.get(self.qualifier.find_by_name(name)?)
    }

    pub fn get_mut_value_by_name(&mut self, name: &String) -> Option<&mut Value> {
        self.values.get_mut(self.qualifier.find_by_name(name)?)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool)
}
