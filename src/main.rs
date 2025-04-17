use std::{fmt::Display, rc::Rc};


trait LoggerTrait {
    fn add_log(&mut self, log_entry: &LogEntry);
    fn display(&self);
    fn filter_by_level(&self, level: LogLevel) -> Vec<&LogEntry>;
}

#[derive(PartialEq)]
enum LogLevel {
    Info,
    Warning,
    Error
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Info => write!(f, "Info")
        }
    }
}

struct LogEntry {
    level: Rc<LogLevel>,
    message: String,
    service: String,
}


struct Logger {
    entries: Vec<LogEntry>
}

impl Logger {
    fn new() -> Self {
        Logger { entries: vec![] }
    }
}

impl LoggerTrait for Logger {
    fn add_log(&mut self, log_entry: &LogEntry) {
         self.entries.push(LogEntry {
            level: log_entry.level.clone(),
            message: log_entry.message.clone(),
            service: log_entry.service.clone()
        });
    }

    fn display(&self) {
        for entry in &self.entries {
            println!("level: {}", entry.level);
            println!("Message: {}", entry.message);
            println!("Service: {}", entry.service);
        }
    }

    fn filter_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        let entries_by_level: Vec<_> = self.entries.iter().filter(|v| {
            *v.level == level
        }).collect();

        entries_by_level
    }
}



use std::io::{self, BufRead};

fn main() {
    println!("Enter log entries in the format: <level>;<message>;<service>");
    println!("Type 'exit' to stop and show logs.");

    let stdin = io::stdin();

    let mut logs = Logger::new();
    
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        
        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let parts: Vec<&str> = input.trim().split(';').collect();
        if parts.len() != 3 {
            println!("Invalid input format. Try again.");
            continue;
        }

        let (level, message, service) = (parts[0], parts[1], parts[2]);
        println!("Level: {level}, Message: {message}, Service: {service}");

        let level = match level.to_lowercase().as_str() {
            "error" => Rc::new(LogLevel::Error),
            "warning" => Rc::new(LogLevel::Warning),
            "info" => Rc::new(LogLevel::Info),
            _ => {
                println!("Wrong level input");
                continue;
            }
        };
        let new_entry = LogEntry { level, message: message.to_string(), service: service.to_string() };
        logs.add_log(&new_entry);
        
    }

    logs.display();
}

