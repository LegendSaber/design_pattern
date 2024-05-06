
trait Strategy {
    fn do_operation(&self, x: i32, y: i32) -> i32;
}

struct OperationAdd;

impl Strategy for OperationAdd {
    fn do_operation(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

struct OperationSubtract;

impl Strategy for OperationSubtract {
    fn do_operation(&self, x: i32, y: i32) -> i32 {
        x - y
    }
}

struct OperationMultiply;

impl Strategy for OperationMultiply {
    fn do_operation(&self, x: i32, y: i32) -> i32 {
        x * y
    }
}

struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    fn execute_strategy(&self, x: i32, y: i32) -> i32 {
        self.strategy.do_operation(x, y)
    }
}

pub(crate) fn test() {
    let add_context = Context::new(Box::new(OperationAdd));
    let res = add_context.execute_strategy(6, 2);
    println!("execute_strategy: {}", res);

    let subtract_context = Context::new(Box::new(OperationSubtract));
    let res = subtract_context.execute_strategy(6, 2);
    println!("execute_strategy: {}", res);

    let multiply_context = Context::new(Box::new(OperationMultiply));
    let res = multiply_context.execute_strategy(6, 2);
    println!("execute_strategy: {}", res);
}
