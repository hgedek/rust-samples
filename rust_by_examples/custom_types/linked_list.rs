use crate::List::*;

const CONST: i32 = 0;
static LANGUAGE: &str = "LANGUAGE";

enum List {
    Cons(u32, Box<List>),
    Nil
}


impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, value: u32) -> List {
        Cons(value, Box::new(self)) 
    } 

    fn len(&self) -> u32 {

        match *self {
            Cons(_, ref tail ) => 1 + tail.len(),
            Nil => 0
        } 
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(value, ref tail) => format!("{}, {}", value, tail.stringify()),
            Nil => format!("Nil")
        }
    }
} 


fn main() {

    let mut list = List::new();
    
    list = list.prepend(0);
    list = list.prepend(1);
    list = list.prepend(2);

    println!("linked list len: {}", list.len());
    println!("{}", list.stringify());

    println!("{}", LANGUAGE);
    println!("{}", CONST);


}
