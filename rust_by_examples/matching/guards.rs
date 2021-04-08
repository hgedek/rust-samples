struct Pair {
    x: i32,
    y: i32
}

fn main() {
    // destructing if condition => result 
    //
    let pair = (1,2);

    match pair {
        (x, y) if x == y => println!("equals"),
        (x, y) if x != y => println!("not equals"),
        (x, _) if x % 2 == 1 => println!("mod"),
        _ => println!("unknown")
    }

    let result = 10;

    match result {
        i if i == 0 => println!("0"),
        i if i == 1 => println!("1"),
        i if i == 2 => println!("2"),
        i if i == 3 => println!("3"),
        i if i == 4 => println!("4"),
        _ => println!("not usable")
    }

    let p = Pair { x: 0, y: 0};

    match p {
        Pair{x, y} if x != y => println!("first"), 
        Pair{x, y} if x == y => println!("second"),
        _ => println!("third")
    }
}

