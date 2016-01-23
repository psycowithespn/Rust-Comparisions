struct Name {
    name : String
}

trait Printable {
    fn print(&self);
}

impl Printable for Name {
    fn print(&self) {
        println!("{}", self.name);
    }
}

fn main() {
    let printable = Name { name : "Patrick".to_string() };
    printable.print();
}