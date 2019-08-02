use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    next_id: u32,
    todos: Vec<Todo>,
}
impl Data {
    pub fn new() -> Data {
        Data {
            next_id: 0,
            todos: vec![],
        }
    }
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
        self.next_id += 1;
    }

    pub fn add_from_text(&mut self, text: &str) {
        self.add(Todo::new(self.next_id, text.to_string()));
    }

    pub fn edit(&mut self, id: u32, text: &str) {
        for todo in self.todos.iter_mut() {
            if id == todo.get_id() {
                todo.set_text(text.to_string());
            }
        }
    }

    pub fn remove(&mut self, id: u32) {
        self.todos.retain(|todo| todo.get_id() != id);
    }

    pub fn set_priority(&mut self, id: u32, priority: Priority) {
        for x in self.todos.iter_mut() {
            if x.id == id {
                match priority {
                    Priority::HIGH => (x.set_priority(Priority::HIGH)),
                    Priority::MEDIUM => (x.set_priority(Priority::MEDIUM)),
                    Priority::LOW => (x.set_priority(Priority::LOW)),
                };
            }
        }
    }

    pub fn set_state(&mut self, id: u32, state: State) {
        for x in self.todos.iter_mut() {
            if x.id == id {
                match state {
                    State::TODO => (x.set_state(State::TODO)),
                    State::PROGRESS => (x.set_state(State::PROGRESS)),
                    State::DONE => (x.set_state(State::DONE)),
                };
            }
        }
    }
    pub fn get_last_id(&self) -> u32 {
        self.next_id - 1
    }
}
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for todo in self.todos.iter() {
            write!(f, "{}\n", todo).unwrap();
        }
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    text: String,
    state: State,
    priority: Priority,
    creation_date: String,
    last_updated_date: String,
}
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}. {} {} {}",
            self.id, self.text, self.state, self.priority,
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    TODO,
    PROGRESS,
    DONE,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value;
        match &self {
            State::DONE => (value = "DONE"),
            State::PROGRESS => (value = "PROGRESS"),
            State::TODO => (value = "TODO"),
        }
        write!(f, "{}", value)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value;
        match &self {
            Priority::LOW => (value = "LOW"),
            Priority::MEDIUM => (value = "MEDIUM"),
            Priority::HIGH => (value = "HIGH"),
        }
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    LOW,
    MEDIUM,
    HIGH,
}

impl Todo {
    pub fn new(id: u32, text: String) -> Todo {
        let dt = Utc::now().format("%Y-%m-%d %H:%M:%S");
        Todo {
            id,
            text,
            state: State::TODO,
            priority: Priority::MEDIUM,
            creation_date: dt.to_string(),
            last_updated_date: dt.to_string(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.set_last_updated_date();
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
        self.set_last_updated_date();
    }

    pub fn set_last_updated_date(&mut self) {
        let dt = Utc::now().format("%Y-%m-%d %H:%M:%S");
        self.last_updated_date = dt.to_string();
    }
}
