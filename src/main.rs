use std::{fmt::Display, rc::Rc};


trait LoggerTrait {
    fn add_log(&mut self, log_entry: &LogEntry);
    fn display(&self);
    fn filter_by_level(&self, level: LogLevel);
    fn filter_by_service(&self, service: &str);
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

#[derive(Clone)]
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

    fn filter_by_level(&self, level: LogLevel) {
        let entries_by_level: Logger = Logger { 
                entries: self.entries.iter().filter(|v| {
                *v.level == level
            })
            .cloned()
            .collect::<Vec<_>>(),
        };
        entries_by_level.display();
    }
    fn filter_by_service(&self, service: &str) {
        let entries_by_service:Logger = Logger {
                entries:  self.entries.iter().filter(|v| {
                v.service == *service
            })
            .cloned()
            .collect::<Vec<_>>(),
        };
        entries_by_service.display();
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
    println!("Do you want to:");
    println!("1. Display all logs");
    println!("2. Filter by service");
    println!("3. Filter by log level");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Unbale to read the input");
    let input = input.trim();
    match input {
        "1" => {
            logs.display();
        },
        "2" => {
            println!("By which service you want to filter?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            logs.filter_by_service(input.trim());
        }
        "3" => {
            println!("By which level you want to filter?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
                match input.trim().to_lowercase().as_str() {
                    "info" => logs.filter_by_level(LogLevel::Info),
                    "warning" => logs.filter_by_level(LogLevel::Warning),
                    "error" => logs.filter_by_level(LogLevel::Error),
                    _ => println!("You entered an invalid level")
                }
        }
        _ => {
            println!("You entered an invalid input");
            println!("Try again");
        }
    }

}

