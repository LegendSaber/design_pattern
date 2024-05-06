
trait Iterator {
    fn has_next(&self) -> bool;

    fn next(&mut self) -> Option<String>;
}

struct NameRepository {
    names: Vec<String>,
}

impl NameRepository {
    fn new() -> Self {
        NameRepository {
            names: vec![
                "Robert".to_string(),
                "John".to_string(),
                "Julie".to_string(),
                "Lora".to_string(),
            ],
        }
    }

    fn get_names(&self) -> &Vec<String> {
        &self.names
    }
}

struct NameIterator {
    index: usize,
    name_repository: NameRepository,
}

impl NameIterator {
    fn new(name_repository: NameRepository) -> Self {
        NameIterator {
            index: 0,
            name_repository,
        }
    }
}

impl Iterator for NameIterator {
    fn has_next(&self) -> bool {
        self.index < self.name_repository.get_names().len()
    }

    fn next(&mut self) -> Option<String> {
        if self.has_next() {
            let name = self.name_repository.get_names()[self.index].clone();
            self.index += 1;
            Some(name)
        } else {
            return None;
        }
    }
}

trait Container {
    fn create_iterator(self) -> Box<dyn Iterator>;
}

impl Container for NameRepository {

    fn create_iterator(self) -> Box<dyn Iterator> {
        Box::new(NameIterator::new(self))
    }
}

pub(crate) fn test() {
    let nr = NameRepository::new();
    let mut container = nr.create_iterator();

    while container.has_next() {
        println!("{}", container.next().unwrap());
    }
}
