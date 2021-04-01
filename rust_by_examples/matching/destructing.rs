#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32)
}

fn main() {

    let tuple = (0, 1, 2);

    match tuple {
        (2,3,4) => println!("direct match"),
        (0, x , y ) => println!("destructing : {}", x + y),
        (3, .. ) => println!("starts with 3 others not important"),
        _ => println!("others")
    }

    let color = Color::RGB(120, 47, 33);

    match color {
        Color::Red => println!("color is red"),
        Color::Green => println!("color is green"),
        Color::Blue => println!("color is blue"),

        // destructing
        Color::RGB(r, g, b) => println!("color is rgb : {} {} {}", r, g, b),
        Color::HSV(h, s, v) => println!("color is hsv: {} {} {}", h, s, v),
        Color::HSL(h, s, l) => println!("color is hsl: {} {} {}", h, s, l),
        Color::CMY(c, m, y) => println!("color is cmy: {} {} {}", c, m, y)
    }

    // &int
    let reference = &4;
    
    match reference {
        &val => println!("&val")
    }

    match *reference {
        val => println!("val")
    }

    let ref reference = 4;

    match reference {
        ref val => println!("ref val: {}", *val)
    }

    let not_reference = 10;

    match not_reference {
        // &val => println!("&val")
        val => println!("val : {}", val)
    }
   
    match not_reference {
        ref val => println!("ref val : {}", val)
    }

    let mut value = 10;

    match value {
        ref mut m => {
            *m = 11;
        }
    }

    println!("value {}", value);

}

