fn main() {

    // defining a variable
    println!("-------defining a variable");
    println!("Hello, Hooman!");
    let mut x = 45; // all variables initially are immutable otherwise it is mentioned
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);
    let y: i64;
    y = 734;
    println!("{}", y);

    // if statement
    println!("-------if statement");
    if y < 10 {
        println!("The {} is less!", y);
    } else {
        println!("The {} is big!", y);
    }

    // loop
    println!("-------loop");
    let mut n = 0;
    loop {
        n += 7;
        if n % 5 == 0 || n % 2 == 0 {
            continue;
        }
        println!("The value of n is {}", n);
        if n > 100 {
            break;
        }
    }

    // for loop
    println!("-------for loop");
    for i in 1..10 {
        println!("The number is {}", i);
    }

    let range = 10..20;
    for i in range {
        println!("element in range {}", i);
    }

    let family_name = vec!["Amir", "Hooman", "Aref", "Shahnaz", "Vihan", "Shima"];

    for name in family_name.iter() {
        println!("Family person is {}", name);
    }

    for (index, name) in family_name.iter().enumerate() {
        println!("Family people {} is {}", index+1, name);
    }

    for name in family_name { // in this way we cannot use family_name next time
        println!("name is {}", name);
    }

    // enum
    println!("-------enum");
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    let player_direction1:Direction = Direction::Up;
    let player_direction2:Direction = Direction::Down;
    let player_direction3:Direction = Direction::Left;
    let player_direction4:Direction = Direction::Right;

    match player_direction1 {
        Direction::Up => println!("We are heading Up!"),
        Direction::Down => println!("We are heading Down!"),
        Direction::Left => println!("We are heading Left!"),
        Direction::Right => println!("We are heading Right!")
    }
    match player_direction2 {
        Direction::Up => println!("We are heading Up!"),
        Direction::Down => println!("We are heading Down!"),
        Direction::Left => println!("We are heading Left!"),
        Direction::Right => println!("We are heading Right!")
    }
    match player_direction3 {
        Direction::Up => println!("We are heading Up!"),
        Direction::Down => println!("We are heading Down!"),
        Direction::Left => println!("We are heading Left!"),
        Direction::Right => println!("We are heading Right!")
    }
    match player_direction4 {
        Direction::Up => println!("We are heading Up!"),
        Direction::Down => println!("We are heading Down!"),
        Direction::Left => println!("We are heading Left!"),
        Direction::Right => println!("We are heading Right!")
    }

    // constants
    println!("-------constants");
    const MAXIMUM_NUMBER: u8 = 7; // must be uppercase
    
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }

    // tuples
    println!("-------tuples");
    let tup1 = ("A", ("Hooman", "Hesamyan"), "C", 734, true);
    println!("{}", (tup1.1).1); // referencing a tuple inside the tuple
    println!("{}", tup1.0);
    println!("{}", tup1.2);
    println!("{}", tup1.3);
    println!("{}", tup1.4);

    let (x, y, z, u, v) = tup1; // destructuring the tuple
    println!("{}", x);
    println!("{}", y.0);

    // function
    println!("-------functions");
    fn count_to(num: u32) {
        for i in 1..num {
            if is_even(i) {
                println!("{} is even", i);
            } else {
                println!("{} is odd", i);
            }
        }
    }

    count_to(7);

    fn is_even(num: u32) -> bool {
        return num % 2 == 0;
    }
    let number = 12;

    println!("is {} even? {}", number, is_even(number));

    // reference
    println!("-------references");
    let x = 7;
    let x_ref = &x;
    println!("x is {}", x);
    println!("x reference is {}", x_ref);


}  
