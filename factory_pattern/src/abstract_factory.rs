
trait Shape {
    fn draw(&self) {
        println!("[+]Shape draw");
    }
}

struct Square;
impl Shape for Square {
    fn draw(&self) {
        println!("[+]Square draw");
    }
}

struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("[+]Circle draw");
    }
}

struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("[+]Rectangle draw");
    }
}

trait Color {
    fn fill(&self) {
        println!("[+]Color fill");
    }
}

struct Red;

impl Color for Red {
    fn fill(&self) {
        println!("[+]Red fill");
    }
}

struct Blue;

impl Color for Blue {
    fn fill(&self) {
        println!("[+]Blue fill");
    }
}

struct Green;

impl Color for Green {
    fn fill(&self) {
        println!("[+]Green fill");
    }
}

trait AbstractFactory {
    fn create_shape(&self, shape: &str) -> Option<Box<dyn Shape>>;
    fn create_color(&self, color: &str) -> Option<Box<dyn Color>>;
}

struct ShapeFactory;

impl ShapeFactory {
    fn new() -> Self {
        ShapeFactory {}
    }
}

impl AbstractFactory for ShapeFactory {
    fn create_shape(&self, shape: &str) -> Option<Box<dyn Shape>> {
        match shape {
            "square" => Some(Box::new(Square)),
            "circle" => Some(Box::new(Circle)),
            "rectangle" => Some(Box::new(Rectangle)),
            _ => {
                println!("[-]Invalid shape type");
                None
            },
        }
    }

    fn create_color(&self, _: &str) -> Option<Box<dyn Color>> {
        None
    }
}

struct ColorFactory;

impl ColorFactory {
    fn new() -> Self {
        ColorFactory {}
    }
}

impl AbstractFactory for ColorFactory {

    fn create_shape(&self, _: &str) -> Option<Box<dyn Shape>> {
        None
    }

    fn create_color(&self, color: &str) -> Option<Box<dyn Color>> {
        match color {
            "red" => Some(Box::new(Red)),
            "blue" => Some(Box::new(Blue)),
            "green" => Some(Box::new(Green)),
            _ => {
                println!("[-]Invalid color type");
                None
            },
        }
    }
}

struct FactoryProducer;

impl FactoryProducer {
    pub fn create_factory(choice: &str) -> Option<Box<dyn AbstractFactory>> {
        match choice {
            "shape" => {
                println!("[+]create shape factory");
                Some(Box::new(ShapeFactory::new()))
            },
            "color" => {
                println!("[+]create color factory");
                Some(Box::new(ColorFactory::new()))
            },
            _ => {
                println!("[-]Invalid factory type");
                None
            }
        }
    }
}

pub(crate) fn test() {
    let shape_factory = FactoryProducer::create_factory("shape").unwrap();

    let square = shape_factory.create_shape("square").unwrap();
    square.draw();
    let circle = shape_factory.create_shape("circle").unwrap();
    circle.draw();
    let rectangle = shape_factory.create_shape("rectangle").unwrap();
    rectangle.draw();

    let color_factory = FactoryProducer::create_factory("color").unwrap();

    let red = color_factory.create_color("red").unwrap();
    red.fill();
    let blue = color_factory.create_color("blue").unwrap();
    blue.fill();
    let green = color_factory.create_color("green").unwrap();
    green.fill();
}
