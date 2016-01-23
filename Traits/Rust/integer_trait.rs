trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("{}", self);
    }
}

fn main() {
    let printable = 654i32;
    printable.print();
}