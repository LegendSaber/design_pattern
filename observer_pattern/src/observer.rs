use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self);
}

struct BinaryObserver {
    subject: Rc<RefCell<Subject>>,
}

impl BinaryObserver {
    fn new(subject: Rc<RefCell<Subject>>) -> Rc<RefCell<Self>> {
        let observer = Rc::new(RefCell::new(BinaryObserver {
            subject: subject.clone(),
        }));

        let mut subject = subject.borrow_mut();
        subject.attach(observer.clone());

        observer
    }
}

impl Observer for BinaryObserver {
    fn update(&self) {
        let subject = self.subject.borrow();
        println!("[+]BinaryObserver state: {}", subject.get_state());
    }
}

struct OctalObserver {
    subject: Rc<RefCell<Subject>>,
}

impl OctalObserver {
    fn new(subject: Rc<RefCell<Subject>>) -> Rc<RefCell<Self>> {
        let observer = Rc::new(RefCell::new(OctalObserver {
            subject: subject.clone(),
        }));

        let mut subject = subject.borrow_mut();
        subject.attach(observer.clone());

        observer
    }
}

impl Observer for OctalObserver {
    fn update(&self) {
        println!("[+]OctalObserver state: {}", self.subject.borrow().get_state());
    }
}

struct HexObserver  {
    subject: Rc<RefCell<Subject>>,
}

impl HexObserver  {
    fn new(subject: Rc<RefCell<Subject>>) -> Rc<RefCell<Self>> {
        let observer = Rc::new(RefCell::new(HexObserver {
            subject: subject.clone(),
        }));

        let mut subject = subject.borrow_mut();
        subject.attach(observer.clone());

        observer
    }
}

impl Observer for HexObserver {
    fn update(&self) {
        println!("[+]OctalObserver state: {}", self.subject.borrow().get_state());
    }
}

struct Subject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    state: i32,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: vec![],
            state: 0,
        }
    }

    fn get_state(&self) -> i32 {
        self.state
    }

    fn set_state(&mut self, state: i32) {
        self.state = state;
        // self.notify_all_observers();
    }

    fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify_all_observers(&self) {
        for observer in &self.observers {
            let observer = observer.borrow();
            observer.update();
        }
    }
}

pub(crate) fn test() {
    let subject = Rc::new(RefCell::new(Subject::new()));

    BinaryObserver::new(subject.clone());
    OctalObserver::new(subject.clone());
    HexObserver::new(subject.clone());

    println!("First state change: 15");
    subject.borrow_mut().set_state(15);
    subject.borrow().notify_all_observers();

    println!("First state change: 20");
    subject.borrow_mut().set_state(20);
    subject.borrow().notify_all_observers();
}
