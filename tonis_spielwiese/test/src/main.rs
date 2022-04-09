struct P {
    name: String,
}

impl P {
    fn new(name: String) -> Self {
        P { name }
    }

    fn say_your_name(&self) {
        println!("Henlo my name is: {}", self.name);
    }
}

fn main() {
    let mut i = 0;
    i = 42;
    let v = vec![1, 2, 3];
    let tuple = (1, 'a', 2., vec![1]);
    take_vec(v);
    let p = P::new(String::from("tofewa"));
    p.say_your_name()
}

fn take_vec(v: Vec<i32>) {
    println!("{:?}", v);
}

