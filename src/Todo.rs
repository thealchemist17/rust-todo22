use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: String,
    text: String,
    //state: State,
    // priority: Priority,
    // creation_date: String,
    // last_updated_date: String,
}
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}. {}", self.id, self.text)
    }
}
// pub enum State {
//     TODO,
//     PROGRESS,
//     DONE,
// }

// pub enum Priority {
//     LOW,
//     MEDIUM,
//     HIGH,
//

impl Todo {
    pub fn new(id: String, text: String) -> Todo {
        Todo { id, text }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}
