pub struct Todo {
    id: String,
    state: State,
    priority: Priority,
    creation_date: String,
    last_updated_date: String,
}

pub enum State {
    TODO,
    PROGRESS,
    DONE,
}

pub enum Priority {
    LOW,
    MEDIUM,
    HIGH,
}

impl Todo {
    pub fn new(
        id: String,
        state: State,
        priority: Priority,
        creation_date: String,
        last_updated_date: String,
    ) -> Todo {
        Todo {
            id,
            state,
            priority,
            creation_date,
            last_updated_date,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn set_state(&mut self, s: State) {
        self.state = s;
    }
    fn set_priority(&mut self, p: Priority) {
        self.priority = p;
    }
    fn set_creation_date(&mut self, creation_date: String) {
        self.creation_date = creation_date;
    }
    fn set_last_updated_date(&mut self, last_updated_date: String) {
        self.last_updated_date = last_updated_date;
    }
}
