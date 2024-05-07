use std::collections::HashMap;


trait Shape {
    fn draw(&self);
}

#[derive(Clone)]
struct ShapeValue {
    id: Option<String>,
    shape_type: Option<String>,
}

impl ShapeValue {
    fn new(shape_type: String) -> Self {
        ShapeValue {
            id: None,
            shape_type: Some(shape_type),
        }
    }

    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn get_shape_type(&self) -> Option<String> {
        self.shape_type.clone()
    }

    fn set_shape_type(&mut self, shape_type: String) {
        self.shape_type = Some(shape_type);
    }
}

#[derive(Clone)]
struct Rectangle {
    shape_value: ShapeValue,
}

impl Rectangle {
    fn new() -> Self {
        Rectangle {
            shape_value: ShapeValue::new("Rectangle".to_string()),
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("[+]Inside Rectangle::draw() method.");
    }
}

#[derive(Clone)]
struct Circle {
    shape_value: ShapeValue,
}

impl Circle {
    fn new() -> Self {
        Circle {
            shape_value: ShapeValue::new("Circle".to_string()),
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("[+]Inside Circle::draw() method.");
    }
}

struct Square {
    shape_value: ShapeValue,
}

impl Square {
    fn new() -> Self {
        Square {
            shape_value: ShapeValue::new("Square".to_string()),
        }
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("[+] Inside Square::draw() method.");
    }
}

struct ShapeCache {
    shapes: HashMap<String, Box<dyn Shape>>,
}

impl ShapeCache {
    fn new() -> Self {
        ShapeCache {
            shapes: HashMap::new(),
        }
    }

    fn get_shape(&self, shape_id: String) -> &Box<dyn Shape> {
        let cached_shape = self.shapes.get(&shape_id).unwrap();

        cached_shape.clone()
    }

    fn load_cache(&mut self) {
        let mut rectangle = Rectangle::new();
        rectangle.shape_value.set_id("1".to_string());
        self.shapes.insert(rectangle.shape_value.get_id().unwrap(), Box::new(rectangle));

        let mut circle = Circle::new();
        circle.shape_value.set_id("2".to_string());
        self.shapes.insert(circle.shape_value.get_id().unwrap(), Box::new(circle));

        let mut square = Square::new();
        square.shape_value.set_id("3".to_string());
        self.shapes.insert(square.shape_value.get_id().unwrap(), Box::new(square));
    }
}

pub(crate) fn test() {
    let mut shape_cache = ShapeCache::new();
    shape_cache.load_cache();

    let cloned_shape = shape_cache.get_shape("1".to_string());
    cloned_shape.draw();

    let cloned_shape = shape_cache.get_shape("2".to_string());
    cloned_shape.draw();

    let cloned_shape = shape_cache.get_shape("3".to_string());
    cloned_shape.draw();
}
