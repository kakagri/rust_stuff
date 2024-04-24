use std::io;

fn main() {
    /*
    let x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    */

    /*
    // Need to declare variables a mutable to change their values.
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    */

    /*
    // to define a constant you need to precise its type !
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    */

    /*
    let x = 5;
    // shadowing = redefining a variable with the same name
    let x = x + 1;
    {
        let x = x * 2;
        // here x is 12
        println!("The value of x in the inner scope is: {x}");
    }
    // here x is 6
    println!("The value of x is: {x}");
    */

    /*
    // Shadowing allows to define a variable with the same name as a previous one
    // but not necessarily with the same type.
    let spaces = "   ";
    let spaces = spaces.len();
    */

    /* Data types */
    
    /*
    // need to precise the type of the variable since parse can return different 'types'
    let guess: u32 = "42".parse().expect("Not a number !");

    let guess: u128 = 0xff;

    let guess = 2.0;

    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;


    let c = 'z';
    let z: char = 'ℤ';
    let moon = '🌝';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    let a: [i32; 5] = [1,2,3,4,5];

    let a = [3; 5]; // this is -> [3,3,3,3,3]

    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];



    let a = [1,2,3,4,5];
    println!("please enter an array index ser");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("didn't read the line");
    // changing index to be an unsigned integer
    let index: usize = index.trim().parse().expect("index is not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    */

    /* Functions */

    /*
    println!("gm rust");
    //another_function();
    another_function(5);
    print_labeled_measurement(5, "ETH");
    */

    /*
    // statements are instructions that perform some action and do not return a value
    // expressions evaluate to a resulting value
    let y = 6;

    //let x = (let y = 6); // this is not allowed

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y should be 4, let's check: {y}");

    let x = five();
    println!("The value of x should be 5, let's check: {x}");

    let x = plus_one(5);
    println!("The value of x should be 6, let's check: {x}");
    */

    /* Control flow */

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // this should make the program not compile
    /*
    if number {
        println!("how is this working ")
    }*/

    let condition = true;
    // the conditional values should be of the same type otherwise this will fail. 
    let number = if condition { 5 } else { 6 };
    println!("The value of the number should be 5 if condition is true otherwise 6, condition={condition}, number={number}");

    /*
    // this will never stop running
    loop {
        println!("again !");
    }*/

    let mut counter = 0;
    // the result should be 100
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10;
        }
    };
    println!("The result should be 100, let's check: {result}");

    // it's possible to label loops to break the outer one instead of the inner one

    let mut count = 0;
    // this should give 0, 10, 9, 1, 10, 9, 2, 10, 2 and then stop 
    //
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count should be two, let's check: {count}");


    let mut number = 3;
    while number != 0 {
        println!("number is {number}");
        number -= 1;
    }
    println!("lift off ? in crypto we say lfg");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (0..4).rev() {
        println!("{number}");
    }



}

/*
fn another_function() {
    println!("Another function.");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("the measurement is {value} {unit_label}");
}

fn five() -> i32 {
    5
}

// to return the value it should be left without the commas thingy, otherwise it won't
// return anything and the function will return a unit type ()
fn plus_one(x: i32) -> i32 {
    x + 1
}

*/