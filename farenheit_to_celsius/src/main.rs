use std::io;

fn main() {
    let mut  farenheit = String::new();

    println!("This program will convert Farenheit to Celsius!");
    println!("Input the Farenheit here!");


    io::stdin().read_line(&mut farenheit)
        .expect("Failed to read line");

    let farenheit: i32 = farenheit.trim().parse()
        .expect("failed to read line");


    let celsius = far_to_celsius(farenheit);

    println!("The Celsius of {} is {}!!", farenheit, celsius);
}

fn far_to_celsius(f: i32) -> i32 {
    (f - 32) * 5/9  
}