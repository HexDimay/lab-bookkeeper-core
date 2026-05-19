use crate::components::{
    argument::Arg,
    argument::TypeValue,
    entity::{Entity, Value},
    qualifier::Qualifier,
    template::Template,
};

/// `HandleData` - слушатель данных, что позволяет `Commander` осуществлять все требуемые комманды.
pub trait HandleData<A: Arg> {
    fn get_entitys(&self) -> &[Entity<A>];
    fn get_mut_entitys(&mut self) -> &mut [Entity<A>];
    fn get_qualifiers(&self) -> &[Qualifier<A>];
    fn get_mut_qualifiers(&mut self) -> &mut [Qualifier<A>];
    fn get_template(&self) -> &[Template<A>];
    fn get_mut_template(&mut self) -> &mut [Template<A>];
}

/// Трейт, что реализован поумолчанию, и предоставляет интерфейс команд для работы с данными.
pub trait Commander<A: Arg>: HandleData<A> {}
