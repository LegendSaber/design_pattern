
#[derive(Clone)]
struct Memento {
    state: String,
}

impl Memento {
    fn new(state: String) -> Self {
        Memento {
            state
        }
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }
}

struct Originator {
    state: String,
}

impl Originator {
    fn new(state: String) -> Self {
        Originator {
            state
        }
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }

    fn save_state_to_memento(&self) -> Memento {
        Memento::new(self.state.clone())
    }

    fn get_state_from_memento(&mut self, memento: Memento) {
        self.state = memento.get_state();
    }
}

struct CareTaker {
    memento_list: Vec<Memento>,
}

impl CareTaker {
    fn new() -> Self {
        CareTaker {
            memento_list: Vec::new(),
        }
    }

    fn add(&mut self, state: Memento) {
        self.memento_list.push(state);
    }

    fn get(&self, index: usize) -> Memento {
        self.memento_list[index].clone()
    }
}

pub(crate) fn test() {

    let mut care_taker = CareTaker::new();

    let mut originator = Originator::new("State #1".to_string());
    care_taker.add(originator.save_state_to_memento());
    originator.set_state("State #2".to_string());
    care_taker.add(originator.save_state_to_memento());
    originator.set_state("State #3".to_string());
    care_taker.add(originator.save_state_to_memento());
    originator.set_state("State #4".to_string());
    care_taker.add(originator.save_state_to_memento());

    println!("Current State: {}", originator.get_state());
    originator.get_state_from_memento(care_taker.get(0));
    println!("First saved State: {}", originator.get_state());
    originator.get_state_from_memento(care_taker.get(1));
    println!("Second saved State: {}", originator.get_state());
}