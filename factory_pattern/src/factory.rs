
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

struct ShapeFactory;

impl ShapeFactory {
    fn new() -> Self {
        ShapeFactory {}
    }

    pub fn create_shape(&self, shape_type: &str) -> Option<Box<dyn Shape>> {
        match shape_type {
            "square" => Some(Box::new(Square)),
            "circle" => Some(Box::new(Circle)),
            "rectangle" => Some(Box::new(Rectangle)),
            _ => {
                println!("[-]Invalid shape type");
                None
            },
        }
    }
}

pub(crate) fn test() {
    let factory = ShapeFactory::new();
    let shape = factory.create_shape("circle");
    shape.unwrap().draw();

    let shape = factory.create_shape("square");
    shape.unwrap().draw();

    let shape = factory.create_shape("rectangle");
    shape.unwrap().draw();
}
