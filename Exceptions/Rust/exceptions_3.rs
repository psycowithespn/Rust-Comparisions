fn main() {
    let number = "42";
    let result : Result<i32,_> = number.parse();
    match result {
        Ok(n) => println!("{} + 10 = {}", number, n + 10),
        Err(_) => println!("{} is not a number", number)
    }
    
    let not_a_number = "asdf";
    let not_a_result : Result<i32,_> = not_a_number.parse();
    match not_a_result {
        Ok(n) => println!("{} + 10 = {}", number, n + 10),
        //Err(_) => println!("{} is not a number", not_a_number)
    }
    
}