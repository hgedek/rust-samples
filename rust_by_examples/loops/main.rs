#![allow(unreachable_code)]

fn main() {
    let mut count = 0_u32;

    loop {
        count += 1;
        
        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);
        
        if count == 10 {
            println!("break time");
            break;
        }
    }

    let mut count2 = 0_u32;

    let result = loop {
        count2 += 1;

        if count2 == 10 {
            break count2
        }
    };

    assert_eq!(result, 10);

    //tagging the loops  
    'outer: loop {
        'inner: loop {
            break 'outer;
        }
    }

    let mut counter = 0;

    while counter < 10 {
        if counter % 15 == 0 {
            println!("fizzbuzz");
        } else if counter % 5 == 0 {
            println!("fizz");
        } else if counter % 3 == 0 {
            println!("buzz");
        } 
        
        counter += 1;
    }
    
    // [ )
    for index in 0..10 {
        println!("{}", index);
    }

    // []
    for index in 0..=10 {
        println!("{}", index); 
    }

 
    let mut names = vec!["ali", "veli", "deli"];

    // implicitly into_iter
    for name in &names {
        match name {
            &"ali" => println!("hey ali"),
            _ => println!("wtf")
        }
    }
    
    // not move but borrowed
    for name in names.iter() {
        match name {
            &"ali" => println!("hey ali"),
            _ => println!("wtf")
        }
    }

    // .iter: borrows each element 
    // .into_iter: moves collection so not usable again
    // .iter_mut : you can edit
    for name in names.iter_mut() {
        *name = match name {
            &mut "ali" => "alix",
            _ => "hello"
        }  
    } 

    let the_number = 10;

    match the_number {
        1 => println!("wtf"),
        2 => println!("wtf * 2"),
        3 => println!("wtf * 3"),
        10..=20 => println!("wtf 10..=20"),
        _ => println!("wtf _")
    }

    let boolean = true ;

    let binary = match boolean {
        false => 0,
        true => 1
    };

    let triple = (1, 1, 2);
    println!("{:?}", triple);

    match triple {
        (0, 1, 2) => println!("hey"),
        _ => println!("hello")
    }

    match triple {
        (x, y, z) => println!("total: {}", x + y + z), 
        _ => println!("not correct type")
    }


    match triple {
        (0, x, y) => println!("first"), 
        (1, ..) => println!("second"),
        _ => println!("rest")
    }


    let mut cities = vec!["ankara", "aksaray", "istanbul"];
    
    for city in cities.iter_mut() {
        *city = match city {
            &mut "ankara" => "mardin", 
            _ => city
        }
    }
}
