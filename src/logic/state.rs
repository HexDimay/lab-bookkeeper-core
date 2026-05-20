/// Определяет центральное состояние системы, осуществляет подключение к базе данных и прочее.
pub struct State {
    pub(crate) name_db: String,
    pub(crate) database: Option<sqlite::Connection>,
}

impl State {
    pub fn new() -> Self {
        Self {
            name_db: String::new(),
            database: None,
        }
    }
}
