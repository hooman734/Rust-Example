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
    let mut x = 7;
    println!("x is {}", x);
    
    {
        let x_ref_mut = &mut x; // mutable reference should enclosed inside a block
        *x_ref_mut += 7;
        println!("x reference is {}", x_ref_mut);
    }

    let x_ref = &x;

    println!("x is {}", x);
    println!("x reference is {}", x_ref);

    // structs
    println!("-------structs");

    struct Color {
        red: u8, // u8: 0-255
        green: u8,
        blue: u8
    }
    let bg = Color {red: 255, green: 70, blue: 15};
    println!("{}, {}, {}", bg.red, bg.green, bg.blue);

    struct Color2(u8, u8, u8);
    let mut bg2 = Color2(30, 70, 255);
    println!("{}, {}, {}", bg2.0, bg2.1, bg2.2);
    bg2.2 = 40;
    println!("{}, {}, {}", bg2.0, bg2.1, bg2.2);

    // pass by reference
    println!("-------pass by reference");

    fn print_color(c: Color) {
        println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
    }

    fn print_color2(c: &Color2) {
        println!("Color - R:{} G:{} B:{}", c.0, c.1, c.2);
    }

    print_color(bg);
    /*   print_color(bg);  *impossible    */

    print_color2(&bg2);
    print_color2(&bg2);
    print_color2(&bg2); // it is possible to have multile function invocation due to it is called by reference

    // arrays
    println!("-------arrays");

    let sample_array = [1, 3, 5, 7]; // either ways are valid
    let sample_array2: [i32; 4] = [6, 8, 15, 20];
    println!("{}", sample_array[1]);

    for (i, el) in sample_array.iter().enumerate() {
        println!("{}-th element is {}", i, el);
    }

    for i in 0..sample_array2.len() {
        println!("{}", sample_array2[i]);
    }

    let array_of_2 = [2; 10]; // generating an array of 2's with length 10
    for el in array_of_2.iter() {
        println!("{}", el);
    }

    // impl
    println!("-------impl");

    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn print_description(&self) {
            println!("Rectangle: {} x {}", self.width, self.height);
        }
        fn is_square(&self) -> bool{
            return self.width == self.height;
        }
        fn area(&self) -> u32 {
            return self.width * self.height;
        }
        fn perimeter(&self) -> u32 {
            return (self.width + self.height) * 2;
        }
    }

    let rectangle: Rectangle = Rectangle {height: 30, width: 10, };
    rectangle.print_description();
    println!("The given rectangle is square? {}", rectangle.is_square());
    println!("Area is {} and perimeter is {}", rectangle.area(), rectangle.perimeter());

    // Strings
    println!("-------Strings");

    let new_string = "Hello World"; // primitive string
    println!("{}", new_string);

    let mut my_string = String::from("How is it going today?");
    println!("{}", my_string);
    println!("{}", my_string.len());
    println!("{}", my_string.is_empty());
    for token in my_string.split_whitespace() { // there is not in primitive string
        println!("{}-", token)
    }
    println!("Does contain {} 'today' in it? {}", my_string, my_string.contains("today"));
    my_string.push_str(new_string);
    println!("{}", my_string);
    /*   println!("{}", my_string.push_str(new_string))  *impossible */


    // Traits (like interface)
    println!("-------Traits"); 
    
    struct Person {
        name: String,
        age: u32,
    }

    // impl Person {
    //     fn to_string(&self) -> String {
    //         return format!("My name is {} and my age is {}", self.name, self.age);
    //     }
    // }

    impl ToString for Person { // trait "ToString" is implemented for "Person"
        fn to_string(&self) -> String {
            return format!("My name is {} and my age is {}", self.name, self.age);
        }
    }

    let hooman: Person = Person {age: 39, name: String::from("Hesamyan Hooman")};
    println!("{}", hooman.to_string());

    // Custom Traits (like interface)
    println!("-------Custom Traits"); 
    
    trait HasVoiceBox {
        // speak
        fn speak(&self);
        
        // check if can speak
        fn can_speak(&self) -> bool;
    }

    impl HasVoiceBox for Person {
        fn speak(&self) {
            println!("Hello, my name is {} ", self.name);
        }

        fn can_speak(&self) -> bool {
            if self.age > 3 {
                return true;
            } return false;
            
        }
    }

    println!("I am {} and I can speak? {}", hooman.name, hooman.can_speak());
    hooman.speak();

    // Match Operator (like Switch)
    println!("-------Match Operator");

    let number = 11;

    match number {
        1 => println!("It is one!"), // case 1
        2 => println!("it is two!"), // case 2
        3 | 4 => println!("it is three or four!"), // case 3 | 4
        5..=10 => println!("it is between 5 to 10"), // case 5 to 10
        _ => println!("it is out of the range!"), // default
    }

    // read input from console 
    println!("-------read input from console");

    use std::io;
    let mut input = String::new();
    println!("Hey mate! Say something:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You said: {}", input.to_ascii_uppercase());
        },
        Err(e) => println!("Oops! SOmething went wrong: {}", e)
    }

    // Hashmap 
    println!("-------Hashmap");

    use std::collections::HashMap;
    
    // define HashMap
    let mut marks = HashMap::new();

    // add values
    marks.insert("Rust Programming", 96);
    marks.insert("Lua Programming", 100);
    marks.insert("C++ Programming", 90);
    marks.insert("Java Programming", 94);

    // prompt length of the HashMap
    println!("How many subjects are collected there? {}", marks.len());

    // find a subject
    match marks.get("Rust Programming") {
        Some(mark) => println!("You have got {} for that.", mark),
        None => println!("You did not study this subject!"),
    }

    // remove an item
    marks.remove("Java Programming");

    // loop through HashMap
    for (subject, mark) in &marks {
        println!("For {} you have got {}.", subject, mark);
    }

    // check for value
    println!("Did you study C#? {} ", marks.contains_key("C# Programming"));



}  
