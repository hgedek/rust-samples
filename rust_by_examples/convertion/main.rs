use std::convert::From;
use std::fmt;


#[derive(Debug)]
struct Number {
    value: i32
}

#[derive(Debug)]
struct Person {
    name: String, 
    surname: String
}

impl From<i32> for Number {
    fn from(item: i32) -> Number {
        Number { value: item }
    }
}

impl From<String> for Person {
    fn from(fullname: String) -> Person {
        let list: Vec<&str> = fullname.split(' ').collect();
        Person { name: list[0].to_string(), surname: list[1].to_string() }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "person's name: {} surname: {}", self.name, self.surname)
    }
}

#[allow(non_snake_case)]
fn main() {

    let literal = "hakan gedek";
    let str_obj = String::from(literal);
    
    let num = Number::from(300);
    println!("number: {:?}", num);

    let fullname = "hakan gedek".to_string();
    let person: Person = Person::from(fullname);

    println!("person: {:?}", person);


    let person2: Person = str_obj.into();
    println!("person2 : {:?}", person2);

    println!("{}", person.to_string());
    println!("{}", person2.to_string());

    let parsed: i32 = "10".parse().unwrap();
    let other_parsed = "300".parse::<i32>().unwrap();

    println!("{}", parsed);
    println!("{}", other_parsed);
}
