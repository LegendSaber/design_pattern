
trait Packing {
    fn pack(&self) -> String;
}

struct Wrapper;

impl Packing for Wrapper {
    fn pack(&self) -> String {
        "Wrapper".to_string()
    }
}

struct Bottle;

impl Packing for Bottle {
    fn pack(&self) -> String {
        "Bottle".to_string()
    }
}

trait Item {
    fn name(&self) -> Option<String> {
        None
    }
    fn packing(&self) -> Option<Box<dyn Packing>> {
        None
    }
    fn price(&self) -> Option<f32> {
        None
    }
}

struct Burger;

impl Item for Burger {
    fn packing(&self) -> Option<Box<dyn Packing>> {
        Some(Box::new(Wrapper))
    }
}

struct ColdDrink;

impl Item for ColdDrink {
    fn packing(&self) -> Option<Box<dyn Packing>> {
        Some(Box::new(Bottle))
    }
}

struct VegBurger;

impl Item for VegBurger {
    fn name(&self) -> Option<String> {
        Some("Veg Burger".to_string())
    }

    fn price(&self) -> Option<f32> {
        Some(25.0)
    }
}

struct ChickenBurger;

impl Item for ChickenBurger {
    fn name(&self) -> Option<String> {
        Some("Chicken Burger".to_string())
    }

    fn price(&self) -> Option<f32> {
        Some(50.5)
    }
}

struct Coke;

impl Item for Coke {
    fn name(&self) -> Option<String> {
        Some("Coke".to_string())
    }

    fn price(&self) -> Option<f32> {
        Some(30.0)
    }
}

struct Pepsi;

impl Item for Pepsi {
    fn name(&self) -> Option<String> {
        Some("Pepsi".to_string())
    }

    fn price(&self) -> Option<f32> {
        Some(35.0)
    }
}

struct Meal {
    items: Vec<Box<dyn Item>>,
}

impl Meal {
    fn new() -> Self {
        Meal { items: vec![] }
    }

    fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    fn get_cost(&self) -> f32 {
        self.items.iter().map(|item| item.price().unwrap_or(0.0)).sum()
    }

    fn show_items(&self) {
        for item in self.items.iter() {
            if let Some(name) = item.name() {
                print!("Item: {}", name);
            }
            if let Some(packing) = item.packing() {
                print!(", Packing: {}", packing.pack());
            }
            if let Some(price) = item.price() {
                println!(", Price: {}", price);
            }
        }
    }
}

struct MealBuilder;

impl MealBuilder {
    fn prepare_veg_burger(&self) -> Meal {
        let mut meal = Meal::new();
        meal.add_item(Box::new(VegBurger));
        meal.add_item(Box::new(Coke));
        meal
    }

    fn prepare_chicken_burger(&self) -> Meal {
        let mut meal = Meal::new();
        meal.add_item(Box::new(ChickenBurger));
        meal.add_item(Box::new(Pepsi));
        meal
    }
}

pub(crate) fn test() {
    let meal_builder = MealBuilder;

    let veg_meal = meal_builder.prepare_veg_burger();
    println!("[+]Veg meal:");
    veg_meal.show_items();
    println!("Total Cost: {}", veg_meal.get_cost());

    let non_veg_meal = meal_builder.prepare_chicken_burger();
    println!("\n[+]Non-Veg Meal:");
    non_veg_meal.show_items();
    println!("Total Cost: {}", non_veg_meal.get_cost());
}
