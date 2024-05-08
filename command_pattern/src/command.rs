
trait  Order {
    fn execute(&self);
}

#[derive(Clone)]
struct Stock {
    name: String,
    quantity: i32,
}

impl Stock {
    fn new(name: String, quantity: i32) -> Self {
        Stock { name, quantity }
    }

    fn buy(&self) {
        println!("[+] Stock [ Name: {}, Quantity: {} ] bought", self.name, self.quantity);
    }

    fn sell(&self) {
        println!("[+] Stock [ Name: {}, Quantity: {} ] sold", self.name, self.quantity);
    }
}

struct BuyStock {
    stock: Stock,
}

impl BuyStock {
    fn new(stock: Stock) -> Self {
        BuyStock { stock }
    }
}

impl Order for BuyStock {
    fn execute(&self) {
        self.stock.buy();
    }
}

struct SellStock {
    stock: Stock,
}

impl SellStock {
    fn new(stock: Stock) -> Self {
        SellStock { stock }
    }
}

impl Order for SellStock {
    fn execute(&self) {
        self.stock.sell();
    }
}

struct Broker {
    order_list: Vec<Box<dyn Order>>
}

impl Broker {
    fn new() -> Self {
        Broker { order_list: Vec::new() }
    }

    fn take_order(&mut self, order: Box<dyn Order>) {
        self.order_list.push(order);
    }

    fn place_orders(&mut self) {
        for order in self.order_list.iter() {
            order.execute();
        }
        self.order_list.clear();
    }
}

pub(crate) fn test() {
    let abc_stock = Stock::new("ABC".to_string(), 10);

    let buy_stock_order = Box::new(BuyStock::new(abc_stock.clone()));
    let sell_stock_order = Box::new(SellStock::new(abc_stock.clone()));

    let mut broker = Broker::new();
    broker.take_order(buy_stock_order);
    broker.take_order(sell_stock_order);

    broker.place_orders();
}
