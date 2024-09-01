use std::io::{stdout, BufWriter, Stdout};

use ferris_says::say;

fn main() {

    // Variables as data types

    let stdout: Stdout = stdout();
    let message = String::from("Hello from Portugal!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer /* mutable variable */).unwrap();

    // Strings

    let first_person_name = "Targeryan";
    let second_person_name = "Stark";

    //first_person_name = "arya"; cannot mutate variables


    println!(
        "Hello {firstPerson} and {secondPerson} !",
        secondPerson = second_person_name,
        firstPerson = first_person_name,
);

     // Mutable and imutable

    let /* mut */ some_name: &str; // uncomment to check that the variable mutates in line 36
    // some_name = "Ghost"; // uncomment to check that the variable mutates in line 36

    some_name = "John Snow";

    println!(
        "You know nothin {some_name}!"
    );

    let is_targeryan: bool = true;

    println! (
        "{some_name} is Targeryan: {is_targeryan}."

    );

    // Char

    let my_char: char = 'a';

    println! (
        "The value of my_char is: {my_char}." );

    // Arrays
    
    let wolves = ["Ghost", "Nymeria", "Summer", "Shaggydog", "Grey Wind", "Lady" ];

    println!(
        "The most known wolves are: {}, {}, {}",
        wolves[0], wolves[1], wolves[wolves.len() - 1]);

    // Tuples

    let queens_slayer = ("John Snow", 30);

    println!( 
        "The Queens Slayer, {} is {} years old.",
        queens_slayer.0,
        queens_slayer.1
    );

    // Functions

    let area_value = area(20, 40);
    println!("The area value is: {area_value}.");

    let _who_is_the_king_slayer = print_kings_slayer("Jaime Lanister");

    let area_value2 = area2(100, 40);
    println!("The area2 value is: {area_value2}.");

    // Conditionals

    let x: i32 = 10;
    if x > 5 { // needs to return a boolean. "if x" doesn't work is rust.
        println!("Greater-1");
    } else {
        println!("Smaller-2")
    }

    let result: &str = if x > 5 {"Greater-2"} else {"Smaller-2"};
    println!("Result: {}", result);

    // Loop, While, for

    let mut counter: i32 = 0;

    loop {
        counter += 1;
        println!("Result: {}", result);
        if counter == 10 {
            break;
        }
    };
    
    let result2: &str = if x > 5 {"Greater-3"} else {"Smaller-3"};

    let return_of_loop: i32 = loop {
        counter += 1;
        println!("Result: {}", result2);
        if counter >= 10 {
            break counter; //returns counter
        }
    };
    println!("Retutn of loop: {}", return_of_loop);


    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for el in a {
        print!("The value is: {} \n", el);
    }

    let mut y = 5;
    while y > 0 {
        println!("The value is: {}", y);
        y -= 1;
    }

    // Struct

    let tup: (i32, &str, bool) = (1, "Sansa", true);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    struct User {
        id: i32,
        name: String,
        is_stark: bool,
    }

    let user1 = User {
        id: 1,
        name: String::from("Sansa"),
        is_stark: true,
    };
    print!("USER: {} {} {}", user1.id, user1.name, user1.is_stark);

    // user1.id = 2;  // This cannot be done

    struct Character {
        id: i32,
        name: String,
        is_stark: bool,
    }

    let user1 = User {
        id: 1,
        name: String::from("Sansa"),
        is_stark: true,
    };
    println!("{} {} {}", user1.id, user1.name, user1.is_stark);

    let mut character1 = Character {
        id: 1,
        name: String::from("Sansa"),
        is_stark: true,
    };
    character1.id = 31; // Now we can do this

    println!("CHARACTER: {} {} {}", character1.id, character1.name, character1.is_stark);

    fn new_character(name: String) -> Character {
        Character {
            id: 2,
            name,
            is_stark: false
        }
    }

    let character2 = new_character(String::from("Cersei"));

    let character3 = Character {
        name: String::from("Joffrey"),
        ..character2 // inherits is and is_stark from character2
    };

    println!("character: {} {} {}", character1.id, character1.name, character1.is_stark);
    println!("character: {} {} {}", character2.id, character2.name, character2.is_stark);
    println!("character: {} {} {}", character3.id, character3.name, character3.is_stark);
}

fn area (length: i32, width: i32) -> i32 {
    return length * width;
}

fn print_kings_slayer(name: &str) -> () {
    println!("The king's slayer name is {}.", name);
}

fn area2 (length: i32, width: i32) -> i32 {
    length * width // this the same as a return statement
}
