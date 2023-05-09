use regex::Regex;

fn main() {
    calculator();
}

fn calculator() {
    let re_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // get user input
    println!("Please enter an operation: ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    let caps = re_add.captures(expression.as_str()).unwrap();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition: i32 = left_value + right_value;

    // show results
    println!("Results: {}", addition)
}

#[allow(dead_code)]
fn cicle_loop() {
    let mut counter = 0;

    loop {
        println!("Hello, world!");
        counter += 1;

        if counter == 10 {
            break;
        }
    }
}

#[allow(dead_code)]
fn get_and_show_user_data_in_the_terminal() {
    println!("Please enter your name: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Please enter your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age: u32 = age.trim().parse().unwrap();

    println!("Hello, {}! You have {} years old", name, age);
}

#[allow(dead_code)]
fn conditionals() {
    println!("Please enter your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age: u32 = age.trim().parse().unwrap();

    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a child");
    }
}
