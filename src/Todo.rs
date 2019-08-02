use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    next_id: u32,
    todos: Vec<Todo>,
}
impl Data{
    pub fn new() -> Data {
        Data {
            next_id: 0,
            todos: vec![],
        }
    }
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
        self.next_id +=1;
    } 

    pub fn add_from_text(&mut self, text: &str){
        
        self.add(Todo::new(self.next_id, text.to_string()));
    }

    pub fn edit(&mut self, id: u32, text: &str){
        for todo in self.todos {
            if id == todo.get_id() {
                todo.set_text(text.to_string());
            }
        }
        
    }

    pub fn remove(&mut self, id: u32){
        self.todos.retain(|todo| todo.get_id() != id);
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
            "{}. {} {} {} {} {}",
            self.id,
            self.text,
            self.state,
            self.priority,
            self.creation_date,
            self.last_updated_date
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
        let dt = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Todo {
            id,
            text,
            state: State::TODO,
            priority: Priority::MEDIUM,
            creation_date: dt,
            last_updated_date: dt,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}
