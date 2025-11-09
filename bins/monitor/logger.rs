use std::{any::Any, sync::Mutex};

pub trait Logger {
    fn new() -> Self where Self: Sized;

    fn log(&self, message: &str);

    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn new() -> Self {
        Self
    }

    fn log(&self, message: &str) {
        println!("{}", message);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MemoryLogger {
    entries: Mutex<Vec<String>>,
}

impl MemoryLogger {
    pub fn new() -> Self {
        Self { entries: Mutex::new(Vec::new()) }
    }

    pub fn get_entries(&self) -> Vec<String> {
        self.entries.lock().unwrap().clone()
    }
}

impl Logger for MemoryLogger {
    fn new() -> Self {
        Self::new()
    }

    fn log(&self, message: &str) {
        self.entries.lock().unwrap().push(message.to_string());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
