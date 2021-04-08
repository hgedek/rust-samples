fn age() -> u32 {
    10
}

// @ => match value but it should be this value or follow rule 
// @ value 
// @ range 
// @ ...
fn main() {
    match age() {
        0 => println!("0"),
        n @ 1 => println!("age : {}", n),
        n @ 2..=10 => println!("age : {}", n),
        n => println!("no binding : {}", n)
    }

    match age() {
        0 => println!("0"),
        n if n == 1 => println!("age : {}", n),
        n => println!("no binding: {}", n)
    }
}
