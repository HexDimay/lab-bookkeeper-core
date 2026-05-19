use crate::{components::argument::Arg, err::ErrorComponents};

/// Определитель - это специальная структура, что задаёт общий каркас всем, кто его использует.
/// Т.е. он является условным предшественником шаблона/макета, что определяет внешность сущности.
#[derive(Debug)]
pub struct Qualifier<A: Arg> {
    /// Всегда постоянен и не может быть изменён.
    id: uuid::Uuid,
    arguments: Vec<A>,
}

impl<A: Arg> Qualifier<A> {
    /// Создаёт новый `Qualifier` с уникальным `id`.
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            arguments: Vec::new(),
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn arguments(&self) -> &[A] {
        &self.arguments
    }

    /// Добавляет новый аргумент в определитель.
    /// Возвращает `Ok(())` в случае успеха.
    pub fn push_argument(&mut self, argument: A) -> Result<(), ErrorComponents> {
        let res_find = self.find_by_name(argument.name());
        match res_find {
            Some(_) => return Err(ErrorComponents::ArgumentExists),
            None => self.arguments.push(argument),
        }

        Ok(())
    }

    /// Удаляет аргумент из определителя и возвращает аргумент в случае успеха.
    pub fn del_argument(&mut self, name: &String) -> Option<A> {
        let res_find = self.find_by_name(name);
        match res_find {
            Some(idx) => Some(self.arguments.remove(idx)),
            None => None,
        }
    }

    /// Находит аргумент по имени и возвращает его позицию в определителе.
    pub fn find_by_name(&self, name: &String) -> Option<usize> {
        self.arguments.iter().position(|arg| arg.name() == name)
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.arguments.get(index)
    }
}
