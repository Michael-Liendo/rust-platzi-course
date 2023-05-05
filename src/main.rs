fn main() {
    conditionals();

    get_and_show_user_data_in_the_terminal();
}

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
