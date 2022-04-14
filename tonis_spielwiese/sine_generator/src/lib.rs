pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Person { name }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Goodbye says: {}", self.name);
    }
}