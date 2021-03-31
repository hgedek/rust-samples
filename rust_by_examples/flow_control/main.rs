#[derive(PartialEq, Debug)]
enum Status {
    Open,
    Closed,
    _Nil
}

fn main() {
    let status = Status::Closed;
    use Status::*;
   
    if status == Open {
        println!("Status: {:?}", status);
    } else if status == Closed {
        println!("Status: {:?}", status);
    } else {
        println!("Status: {:?}", status)
    }

    let a = true;
    let b = false;

    if a && b {
        println!("yes");
    }
}
