use std::sync::Mutex;
use lazy_static::lazy_static;

// 线程安全的单例模式

lazy_static! {
    static ref INSTANCE: Mutex<Option<Singleton>> = Mutex::new(None);
}

struct Singleton {
    value: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            value: 0,
        }
    }

    pub fn get_instance() -> &'static Mutex<Option<Singleton>> {
        let mut instance = INSTANCE.lock().unwrap();

        if instance.is_none() {
            *instance = Some(Singleton::new());
        }

        &INSTANCE
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

pub(crate) fn test() {
    let mut guard = Singleton::get_instance().lock().unwrap();
    let obj = guard.as_mut().unwrap();
    println!("Before set. Value: {}", obj.get_value());
    obj.set_value(1900);
    println!("After set. Value: {}", obj.get_value());
}
