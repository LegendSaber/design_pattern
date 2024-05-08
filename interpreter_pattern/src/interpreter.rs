
trait Expression {
    fn interpret(&self, context: String) -> bool;
}

struct TerminalExpression {
    data: String,
}

impl TerminalExpression {
    fn new(data: String) -> Self {
        TerminalExpression { data }
    }
}

impl Expression for TerminalExpression {
    fn interpret(&self, context: String) -> bool {
        context.contains(self.data.as_str())
    }
}

struct AndExpression {
    expression1: Box<dyn Expression>,
    expression2: Box<dyn Expression>,
}

impl AndExpression {
    fn new(expression1: Box<dyn Expression>, expression2: Box<dyn Expression>) -> Self {
        AndExpression {
            expression1,
            expression2,
        }
    }
}

impl Expression for AndExpression {
    fn interpret(&self, context: String) -> bool {
        self.expression1.interpret(context.clone()) && self.expression2.interpret(context.clone())
    }
}

struct OrExpression {
    expression1: Box<dyn Expression>,
    expression2: Box<dyn Expression>,
}

impl OrExpression {
    fn new(expression1: Box<dyn Expression>, expression2: Box<dyn Expression>) -> Self {
        OrExpression {
            expression1,
            expression2,
        }
    }
}

impl Expression for OrExpression {
    fn interpret(&self, context: String) -> bool {
        self.expression1.interpret(context.clone()) || self.expression2.interpret(context.clone())
    }
}

fn get_male_expression() -> OrExpression {
    let terminal_expression1 = TerminalExpression::new("Robert".to_string());
    let terminal_expression2 = TerminalExpression::new("John".to_string());

    OrExpression::new(
        Box::new(terminal_expression1),
        Box::new(terminal_expression2),
    )
}

fn get_married_woman_expression() -> AndExpression {
    let terminal_expression1 = TerminalExpression::new("Julie".to_string());
    let terminal_expression2 = TerminalExpression::new("Married".to_string());

    AndExpression::new(
        Box::new(terminal_expression1),
        Box::new(terminal_expression2),
    )
}

pub(crate) fn test() {
    let is_male = get_male_expression();
    let is_married_woman = get_married_woman_expression();

    println!("John is male? {}", is_male.interpret("John".to_string()));
    println!("Julie is a married women? {}", is_married_woman.interpret("Married Julie".to_string()));
}
