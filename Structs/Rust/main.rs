struct Person {
    age: i32,
    name: String,
}

impl Person {
    fn print(&self) {
        println!("{}: {} years old", self.name, self.age);
    }
}

fn main() {
    let me = Person { name: "Patrick".to_string(), age: 20 };
    me.print();
}