use chrono::Local;

struct ChatRoom;

impl ChatRoom {
    fn show_message(user: &User, message: String) {
        let local = Local::now();
        println!("{} [{}]: {}", local.format("%Y-%m-%d"), user.get_name(), message)
    }
}

struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        User { name }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn send_message(&self, message: String) {
        ChatRoom::show_message(self, message);
    }
}

pub(crate) fn test() {
    let robert = User::new("Robert".to_string());
    let john = User::new("John".to_string());

    robert.send_message("Hi! John!".to_string());
    john.send_message("Hello! Robert!".to_string());
}