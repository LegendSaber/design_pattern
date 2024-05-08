
trait State {
    fn do_action(&self, context: &mut Context);
    fn get_state_name(&self) -> String;
}

#[derive(Clone)]
struct StartState;

impl State for StartState {
    fn do_action(&self, context: &mut Context) {
        println!("Player is in start state");
        context.set_state(Box::new(Self));
    }

    fn get_state_name(&self) -> String {
        "Start".to_string()
    }
}

#[derive(Clone)]
struct StopState;

impl State for StopState {
    fn do_action(&self, context: &mut Context) {
        println!("Player is in stop state");
        context.set_state(Box::new(Self));
    }

    fn get_state_name(&self) -> String {
        "Stop".to_string()
    }
}

struct Context {
    state: Option<Box<dyn State>>,
}

impl Context {
    fn new() -> Self {
        Context {
            state: None
        }
    }

    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = Some(state);
    }

    fn get_state(&self) -> String {
        self.state.as_ref().unwrap().get_state_name()
    }
}

pub(crate) fn test() {
    let start_state = Box::new(StartState);
    let stop_state = Box::new(StopState);

    let mut context = Context::new();

    start_state.do_action(&mut context);
    println!("{}", context.get_state());
    stop_state.do_action(&mut context);
    println!("{}", context.get_state());
}