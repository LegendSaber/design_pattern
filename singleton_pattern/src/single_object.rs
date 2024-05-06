static mut INSTANCE: Option<SingleObject> = None;

// 线程不安全的单例模式

struct SingleObject {
}

impl SingleObject {

    fn new() -> Self {
        SingleObject {
        }
    }

    pub fn get_instance() -> &'static SingleObject {
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(SingleObject::new());
            }

            INSTANCE.as_ref().unwrap()
        }
    }

    pub fn show_message(&self) {
        println!("This is single object message!");
    }
}

pub(crate) fn test() {
    let obj = SingleObject::get_instance();
    obj.show_message();
}
