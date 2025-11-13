use std::sync::{Arc, Mutex};

pub struct DebugLogger {
    pub debug_messages: Vec<String>,
}

impl DebugLogger {
    pub fn new() -> DebugLogger {
        Self {
            debug_messages: vec![],
        }
    }

    pub fn log(&mut self, message: String) {
        self.debug_messages.push(message);
    }

    pub fn safe_log(arc_mutex_logger: &Arc<Mutex<Self>>, message: String) {
        match arc_mutex_logger.lock() {
            Ok(mut logger_guard) => {
                logger_guard.log(message);
            }
            Err(poisoned_error) => {
                let mut logger_guard = poisoned_error.into_inner();
                logger_guard.log(format!("(POISONED MUTEX) {}", message));
            }
        }
    }

    pub fn get_messages(arc_mutex_logger: &Arc<Mutex<Self>>) -> Vec<String> {
        match arc_mutex_logger.lock() {
            Ok(logger_guard) => logger_guard.debug_messages.clone(),
            Err(err) => {
                vec![]
            }
        }
    }
}
