fn main() {
    println!("Hello, world!");

    another_function(23, 32);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 
    };

    println!("The value of y is: {}", y);

    let x = five();

    println! ("{}", x);

    let x = plus_one(five());
    
    println! ("{}", x);
}

fn another_function(x: i32, y: i32) {
    println!("another one!! You have just inputted: {}", x);
    println!("Annnnnd another one!! you have also inputted: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}