use chrono::NaiveDate;

use crate::{
    components::{argument::Arg, qualifier::Qualifier, template::Template},
    err::ErrorComponents,
};

/// `Entity` - основная сущность над которой происходят операции.
pub struct Entity<A: Arg> {
    /// Не может быть изменён.
    id: uuid::Uuid,
    qualifier: Qualifier<A>,
    /// Позиция значения должна совпадать с позицией аргумента в определителе.
    values: Vec<Value>,
}

impl<A: Arg> Entity<A> {
    pub fn new(qualifier: Qualifier<A>) -> Self {
        let values = Self::init_args(&qualifier);
        Self {
            id: uuid::Uuid::new_v4(),
            qualifier,
            values,
        }
    }

    pub fn apply_template(&mut self, templ: &Template<A>) -> Result<(), ErrorComponents> {
        if self.values.len() != templ.values().len() {
            return Err(ErrorComponents::PatternsDontMatch);
        }

        for (i, v_templ) in templ.values().iter().enumerate() {
            if &self.values()[i] != v_templ {
                return Err(ErrorComponents::PatternsDontMatch);
            }
        }

        self.values = templ.values().to_vec();

        Ok(())
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    /// Инициализация значений относительно аргументов.
    fn init_args(qualifier: &Qualifier<A>) -> Vec<Value> {
        qualifier
            .arguments()
            .iter()
            .map(|a| match a.type_value() {
                super::argument::TypeValue::Float => Value::Float(Default::default()),
                super::argument::TypeValue::Int => Value::Int(Default::default()),
                super::argument::TypeValue::String => Value::String(Default::default()),
                super::argument::TypeValue::Bool => Value::Bool(Default::default()),
                super::argument::TypeValue::Date => Value::Date(Default::default()),
            })
            .collect()
    }

    pub fn qualifier(&self) -> &Qualifier<A> {
        &self.qualifier
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    Date(NaiveDate),
}
