
trait AbstractCustomer {
    fn is_null(&self) -> bool;
    fn get_name(&self) -> String;
}

struct RealCustomer {
    name: String,
}

impl RealCustomer {
    fn new(name: String) -> Self {
        RealCustomer { name }
    }
}

impl AbstractCustomer for RealCustomer {
    fn is_null(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct NullCustomer;

impl NullCustomer {
    fn new() -> Self {
        NullCustomer
    }
}

impl AbstractCustomer for NullCustomer {
    fn is_null(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        "Not Available in Customer Database".to_string()
    }
}

struct CustomerFactory {
    names: Vec<String>,
}

impl CustomerFactory {
    fn new(names: Vec<String>) -> Self {
        CustomerFactory {
            names,
        }
    }

    fn get_customer(&self, name: String) -> Box<dyn AbstractCustomer>{
        for customer in self.names.iter() {
            if customer.clone() == name {
                return Box::new(RealCustomer::new(name));
            }
        }

        Box::new(NullCustomer::new())
    }
}

pub(crate) fn test() {
    let factory = CustomerFactory::new(vec!["Rob".to_string(), "Joe".to_string(), "Julie".to_string()]);

    let customer1 = factory.get_customer("Rob".to_string());
    let customer2 = factory.get_customer("Bob".to_string());
    let customer3 = factory.get_customer("Julie".to_string());
    let customer4 = factory.get_customer("Laura".to_string());

    println!("[+]Customers:");
    println!("{}", customer1.get_name());
    println!("{}", customer2.get_name());
    println!("{}", customer3.get_name());
    println!("{}", customer4.get_name());
}