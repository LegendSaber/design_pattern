
trait Image {
    fn display(&mut self);
}

struct RealImage {
    file_name: String,
}

impl RealImage {
    fn new(file_name: String) -> Self {
        println!("Loading image from disk: {}", file_name);
        RealImage { file_name }
    }
}

impl Image for RealImage {
    fn display(&mut self) {
        println!("Displaying image: {}", self.file_name);
    }
}

struct ProxyImage {
    file_name: String,
    real_image: Option<RealImage>,
}

impl ProxyImage {
    fn new(file_name: String) -> Self {
        ProxyImage {
            file_name,
            real_image: None,
        }
    }
}

impl Image for ProxyImage {
    fn display(&mut self) {
        if let None = self.real_image {
            self.real_image = Some(RealImage::new(self.file_name.clone()));
        }
        self.real_image.as_mut().unwrap().display();
    }
}

pub(crate) fn test() {
    let mut image = ProxyImage::new("test_image.jpg".to_string());

    image.display();
    println!();
    image.display();
}
