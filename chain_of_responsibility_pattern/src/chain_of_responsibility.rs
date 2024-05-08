enum Level {
    INFO,
    DEBUG,
    ERROR,
}

trait AbstractLogger {
    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>);
    fn log_message(&self, level: usize, message: String);
    fn write(&self, message: String);
}

struct ConsoleLogger {
    level: usize,
    next_logger: Option<Box<dyn AbstractLogger>>,
}

impl ConsoleLogger {
    fn new(level: usize) -> Self {
        ConsoleLogger {
            level,
            next_logger: None,
        }
    }
}

impl AbstractLogger for ConsoleLogger {
    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>) {
        self.next_logger = Some(next_logger);
    }

    fn log_message(&self, level: usize, message: String) {
        if self.level <= level {
            self.write(message.clone());
        }
        if self.next_logger.is_some() {
            self.next_logger.as_ref().unwrap().log_message(level, message.clone());
        }
    }

    fn write(&self, message: String) {
        println!("Standard Console::Logger: {}", message);
    }
}

struct ErrorLogger {
    level: usize,
    next_logger: Option<Box<dyn AbstractLogger>>,
}

impl ErrorLogger {
    fn new(level: usize) -> Self {
        ErrorLogger {
            level,
            next_logger: None,
        }
    }
}

impl AbstractLogger for ErrorLogger {
    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>) {
        self.next_logger = Some(next_logger);
    }

    fn log_message(&self, level: usize, message: String) {
        if self.level <= level {
            self.write(message.clone());
        }
        if self.next_logger.is_some() {
            self.next_logger.as_ref().unwrap().log_message(level, message.clone());
        }
    }

    fn write(&self, message: String) {
        println!("Error Console::Logger: {}", message);
    }
}

struct FileLogger {
    level: usize,
    next_logger: Option<Box<dyn AbstractLogger>>,
}

impl FileLogger {
    fn new(level: usize) -> Self {
        FileLogger {
            level,
            next_logger: None,
        }
    }
}

impl AbstractLogger for FileLogger {
    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>) {
        self.next_logger = Some(next_logger);
    }

    fn log_message(&self, level: usize, message: String) {
        if self.level <= level {
            self.write(message.clone());
        }
        if self.next_logger.is_some() {
            self.next_logger.as_ref().unwrap().log_message(level,message.clone());
        }
    }

    fn write(&self, message: String) {
        println!("File::Logger: {}", message);
    }
}

fn get_chain_of_loggers() -> Box<dyn AbstractLogger> {
    let mut error_logger = Box::new(ErrorLogger::new(Level::ERROR as usize));
    let mut file_logger = Box::new(FileLogger::new(Level::DEBUG as usize));
    let console_logger = Box::new(ConsoleLogger::new(Level::INFO as usize));

    file_logger.set_next_logger(console_logger);
    error_logger.set_next_logger(file_logger);

    error_logger
}

pub(crate) fn test() {
    let logger = get_chain_of_loggers();

    logger.log_message(Level::INFO as usize, "This is an information.".to_string());
    logger.log_message(Level::DEBUG as usize, "This is a debug level information.".to_string());
    logger.log_message(Level::ERROR as usize, "This is an error information.".to_string());
}
