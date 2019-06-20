fn main() {
    let num1 = 24;
    let num2 = num1;
    println!("{}", num1);

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Self { x, y}
        }
    }

    let x = Point::new(5, 10);

    println!("{:#?}", x);

    fn print_point(point: Point) {
        println!("x: {}, y: {}", point.x, point.y);
    }

    let p1 = Point {x:1, y:2};
    let p2 = p1;
    print_point(p1);
    println!("{}", p1.x);

    fn inc_x(point: &mut Point) {
        point.x += 1;
    }
    
    let mut p1 = Point { x: 1, y: 2};
    inc_x(&mut p1);
    println!("{:#?}", p1);

    let tuple = (24,42);
    println!("{}, {}", tuple.0, tuple.1);

    #[derive(Debug, Clone, Copy)]
    enum Expr {
        Null,
        Add(i32, i32),
        Sub(i32, i32),
        Mul(i32, i32),
        Div { dividend: i32, divisor: i32},
        Val(i32),
    }

    let quotient = Expr::Div{ dividend: 10, divisor: 2};
    let sum = Expr::Add(40, 2);

    fn print_expr (expr: Expr) {
        match expr {
            Expr::Null => println!("No Value"),
            Expr::Add(x, y) => println!("{}", x + y),
            Expr::Sub(x,y) => println!("{}", x-y),
            Expr::Mul(x,y) => println!("{}", x*y ),
            Expr::Div {dividend: x, divisor: 0} => println! ("Divisor is Zero"),
            Expr::Div {dividend: x, divisor: y} => println! ("{}", x/y),
            Expr::Val(x) => println!("{}", x),
        }
    }
    
    fn uppercase(c: u8) -> u8 {
        match c {
            b'a'...b'z' => c - 32,
            _ => c,
        }
    }

    fn is_alphanumeric(c: char) -> bool {
        match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false,
        }
    }

    println! ("{}", uppercase(b'a') as char);
    let b = is_alphanumeric('A');
    println!("{}", b);

    let mut tuple = (20, 24);
    let (a, b) = tuple;
    println!("{}, {}", a, b);
    
    trait BitSet {
        fn clear(&mut self, index: usize);
        fn is_set(&self, index: usize) -> bool;
        fn set(&mut self, index: usize);
    }

    impl BitSet for u64 {
        fn clear(&mut self, index: usize) {
            *self &= !(1 << index);
        }

        fn is_set(&self, index: usize) -> bool {
            (*self >> index) & 1 == 1
        }

        fn set(&mut self, index: usize) {
            *self |= 1 << index;
        }
    }

    let mut num = 0;
    num.set(15);
    println!("{}", num.is_set(15));
    num.clear(15);
}
