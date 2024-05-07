use std::collections::HashMap;
use rand::random;

trait Shape {
    fn draw(&self);
}

struct Circle {
    color: String,
    x: u32,
    y: u32,
    radius: u32,
}

impl Circle {
    fn new(color: String, x: u32, y: u32, radius: u32) -> Self {
        Circle {
            color,
            x,
            y,
            radius,
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("[+]Circle: Draw() [Color : {}, x: {}, y: {}, radius: {}]", self.color, self.x, self.y, self.radius);
    }
}

struct ShapeFactory {
    circle_map: HashMap<String, Box<dyn Shape>>,
}

impl ShapeFactory {
    fn new() -> Self {
        ShapeFactory {
            circle_map: HashMap::new(),
        }
    }

    fn get_circle(&mut self, color: String, x: u32, y: u32, radius: u32) -> &Box<dyn Shape> {
        if !self.circle_map.contains_key(&color) {
            let circle = Box::new(Circle::new(color.clone(), x, y, radius));
            self.circle_map.insert(color.clone(), circle);
            println!("Creating circle of color: {}", color.clone());
        }
        self.circle_map.get(&color).unwrap().clone()
    }
}

pub(crate) fn test() {

    let mut shape_factory = ShapeFactory::new();

    for _ in 0.. 20 {
        let circle = shape_factory.get_circle(get_random_color(), get_random(), get_random(), 100);
        circle.draw();
    }
}

fn get_random_color() -> String {
    let colors = ["Red", "Green", "Blue", "White", "Black"];

    colors[random::<usize>() % colors.len()].to_string()
}

fn get_random() -> u32 {
    random::<u32>() % 100
}
