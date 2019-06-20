fn main() {
    let number = 90;

    if number != 0 {
        println!("condition was true");
    } else {
        println!("condition was false!!");
    }

    let number = 8;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisble by 2");
    } else {
        println!("Number aint divisble by 2 3 or 4!! idiot or whaaaat");
    }

    let condition = false;
    let number = if condition {
        5
    } else {
        9
    };

    println!("The value of the number is: {} mothafuckaaaa", number);

    // loop {
    //     println!("again!");
    // }

    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number = number -1;
    }

    println!("LIFT OFF!!!");

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF AGAIN!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
