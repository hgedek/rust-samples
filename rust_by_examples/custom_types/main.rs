#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String, 
    age: u8,
}

struct Unit;

#[derive(Debug)]
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32 
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point, 
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 }
}

fn inspect(event: WebEvent ) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed : {}", c),
        WebEvent::Paste(s) => println!("passted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={} y={}", x, y);
        },
    }
}

fn foo(event: WebEvent) {
    match event {
        WebEvent::KeyPress(key) => println!("pressed : {}", key), 
        WebEvent::Paste(text) => println!("pasted: {}", text), 
        WebEvent::Click { x, y } => println!("x: {} y: {}", x, y), 
        _ => println!("others")
    }
}

struct Ooooooooooooooo {
    name: String
}

enum Odsfdsfdsfdsafdsfadsfdsfsaf {
    A,
    B,
    C
}

// aliasing
type Oo = Ooooooooooooooo;
type Df = Odsfdsfdsfdsafdsfadsfdsfsaf;

enum Verbose {
    Add, 
    Substract
}

impl Verbose {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Verbose::Add => x + y,
            Verbose::Substract => x - y
        } 
    } 
}


enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Soldier,
}

enum Number {
    One,
    Two,
    Three
}

fn main() {
    
    let name = String::from("hakan");
    let age = 30;
    let person = Person { name, age };

    println!("person: {:?}", person);

    let point: Point = Point { x: 10.4, y: 33.4 };
    println!("point coords: ({},{})", point.x, point.y);

    let bottom_right = Point {x: 3.3, ..point };
    println!("bottom : ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x : top_edge, y: left_edge } = point; // binding
   
    println!("top_edge: {}", top_edge);

    let rectangle = Rectangle {
        top_left : Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    let unit = Unit;
    
    let pair = Pair(1, 0.3);

    println!("pair : {:?}", pair);

    let Pair( a, b) = pair;

    println!("a : {} b: {}", a, b);

    
    let pl = WebEvent::PageLoad;
    let pul = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("pasted string".to_owned());
    let click = WebEvent::Click { x: 10, y: 20 };
    
    inspect(pl);
    inspect(pul);
    inspect(pressed);
    inspect(pasted);
    inspect(click);

    
    let p1 = Point {x : 0.0, y: 0.0 };
    let Point { x: my_x, y: my_y } = p1;


    let w1 = WebEvent::KeyPress('a');
    let w2 = WebEvent::Paste("paste".to_owned());
    let w3 = WebEvent::Click {x : 0, y: 0 };

    foo(w1);
    foo(w2);
    foo(w3);

    let o = Oo { name : "hakan".to_string() };

    let a = Df::A;
    let b = Df::B;
    let c = Df::C;

    let v = Verbose::Add;
    println!("result1 : {}", v.run(1, 2));
    
    let v2 = Verbose::Substract;
    println!("result : {}", v2.run(3,1));


    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Poor => println!("being poor"),
        Rich => println!("being rich")
    }

    match work {
        Soldier => println!("being soldier"),
        Civilian => println!("being civilian")
    }

    println!("One: {}", Number::One as i32);
    println!("Two: {}", Number::Two as i32);
    println!("Three: {}/", Number::Three as i32);
}

