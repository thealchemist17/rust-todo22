use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: String,
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
    pub fn new(id: String, text: String) -> Todo {
        let dt = Utc
            .ymd(Utc::now().year(), Utc::now().month(), Utc::now().day())
            .and_hms(Utc::now().hour(), Utc::now().minute(), Utc::now().second());
        Todo {
            id,
            text,
            state: State::TODO,
            priority: Priority::MEDIUM,
            creation_date: dt.to_string(),
            last_updated_date: dt.to_string(),
        }
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
