use crate::components::{argument::Arg, entity::Value, qualifier::Qualifier};

/// `Template` - структура-шаблон, созданная для автоматического заполнения `Entity`.
pub struct Template<A: Arg> {
    /// Не может быть изменён.
    id: uuid::Uuid,
    qualifier: Qualifier<A>,
    /// Позиция значения должна совпадать с позицией аргумента в определителе.
    values: Vec<Value>,
}

impl<A: Arg> Template<A> {
    pub fn new(qualifier: Qualifier<A>) -> Self {
        let values = Self::init_args(&qualifier);
        Self {
            id: uuid::Uuid::new_v4(),
            qualifier,
            values,
        }
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
}
