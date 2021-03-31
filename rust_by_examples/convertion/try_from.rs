use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct Person {
    name: String, 
    surname: String
}

impl TryFrom<String> for Person {
    type Error = ();

    fn try_from(fullname: String) -> Result<Self, Self::Error> {
        if fullname.is_empty() {
            Err(())
        } else {
            let list: Vec<&str> = fullname.split(' ').collect();
            if list.len() != 2 {
                Err(())
            } else {
                Ok(Person { name: list[0].to_string(), surname: list[0].to_string()})
            }
        } 
    }
}

fn main() {
    let fullname = "hakan gedek".to_string();
    let person = Person::try_from(fullname);
    println!("person: {:?} ", person);

    let othername = "hakan".to_string();
    let result: Result<Person, ()> = othername.try_into();
    assert_eq!(result, Err(()));
}
