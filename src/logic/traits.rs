use crate::components::{
    argument::Arg,
    argument::TypeValue,
    entity::{Entity, Value},
    qualifier::Qualifier,
    template::Template,
};

/// `HandleData` - слушатель данных, что позволяет `Commander` осуществлять все требуемые комманды.
pub trait HandleData<A: Arg> {
    fn get_slice_entitys(&self) -> &[Entity<A>];
    fn get_mut_slice_entitys(&mut self) -> &mut [Entity<A>];
    fn get_slice_qualifiers(&self) -> &[Qualifier<A>];
    fn get_mut_slice_qualifiers(&mut self) -> &mut [Qualifier<A>];
    fn get_slice_template(&self) -> &[Template<A>];
    fn get_mut_slice_template(&mut self) -> &mut [Template<A>];

    fn get_vec_entitys(&self) -> &Vec<Entity<A>>;
    fn get_mut_vec_entitys(&mut self) -> &mut Vec<Entity<A>>;
    fn get_vec_qualifiers(&self) -> &Vec<Qualifier<A>>;
    fn get_mut_vec_qualifiers(&mut self) -> &mut Vec<Qualifier<A>>;
    fn get_vec_template(&self) -> &Vec<Template<A>>;
    fn get_mut_vec_template(&mut self) -> &mut Vec<Template<A>>;
}

/// Трейт, что реализован поумолчанию, и предоставляет интерфейс команд для работы с данными.
/// # TODO:
/// - [ ] Реализация проверки конфигураций и прочего необходимого для начала работы (init).
/// - [ ] Реализация полной очистки или определённых данных.
/// - [ ] Чтение и запись в базу данных.
/// - [ ] Создание специфичных баз данных.
/// - [ ] Формирование истории действий (транзакций)
/// - [ ] Сбор статистики (хз, может перекинуть это на клиент)
/// - [ ] Редактирование определённых частей данных
pub trait Commander<A: Arg>: HandleData<A> {}
