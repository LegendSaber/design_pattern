
trait DrawAPI {
    fn draw_circle(&self, radius: i32, x: i32, y: i32);
}

struct RedCircle;

impl DrawAPI for RedCircle {
    fn draw_circle(&self, radius: i32, x: i32, y: i32) {
        println!("[+]Drawing Circle[ color: red, radius: {}, x: {}, y:{} ]", radius, x, y);
    }
}

struct GreenCircle;

impl DrawAPI for GreenCircle {
    fn draw_circle(&self, radius: i32, x: i32, y: i32) {
        println!("[+]Drawing Circle[ color: green, radius: {}, x: {}, y:{} ]", radius, x, y);
    }
}

trait Shape {
    fn draw(&self);
}

struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    draw_api: Box<dyn DrawAPI>,
}

impl Circle {
    fn new(x: i32, y: i32, radius: i32, draw_api: Box<dyn DrawAPI>) -> Self {
        Circle {
            x,
            y,
            radius,
            draw_api,
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        self.draw_api.draw_circle(self.radius, self.x, self.y);
    }
}

pub(crate) fn test() {
    let red_circle = Circle::new(1, 2, 3, Box::new(RedCircle));
    let green_circle = Circle::new(5, 7, 11, Box::new(GreenCircle));

    red_circle.draw();
    green_circle.draw();
}
