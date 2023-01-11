use std::io;
fn main() {
    check_prime_numbers();
    shortest_path();
    patterns_0_1_s();
    unique_numbers();
    fizz_buzz();
}

fn check_prime_numbers() {
    println!("############# check prime numbers");
    let mut num = String::new();
    println!("Enter the number: ");
    io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();
    let mut res = true;

    for checker in 2..num {
        if num % checker == 0 {
            res = false;
            break;
        }
    }
    if res {
        println!("Number is Prime");
    } else {
        println!("Not Prime");
    }
}

fn shortest_path() {
    println!("############# find the shortest path");
    println!(
        "
    Problem - Find the displacement              N ^
                                                   | 
    Input String: NNNSSEEWE                        |
    Output : NEE                                   |
                                          W<-------o------->E
                                              (0,0)|
                                                   |
                                                   |
                                                 S v
"
    );
    let mut path = String::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    println!("Enter your path: ");
    io::stdin().read_line(&mut path).unwrap();

    for direction in path.trim().chars() {
        if direction == 'N' || direction == 'n' {
            y += 1;
        } else if direction == 'S' || direction == 's' {
            y -= 1;
        } else if direction == 'E' || direction == 'e' {
            x += 1;
        } else if direction == 'W' || direction == 'w' {
            x -= 1;
        }
    }

    println!("Final Displacement: x={} y={}", x, y);

    print!("Shortest Path: ");
    for _ in 0..x.abs() {
        if x > 0 {
            print!("E");
        } else {
            print!("W");
        }
    }
    for _ in 0..y.abs() {
        if y > 0 {
            print!("N");
        } else {
            print!("S");
        }
    }
    println!();
}

/*
    Observations:
    - For ith line print i numbers
    - odd line start with 0, and value alternates btw 0 and 1
    - even line start with 1, and value alternates btw 0 and 1
    - print n lines
*/
fn patterns_0_1_s() {
    println!(
        "############# Print patterns

    0
    10
    010
    1010
    01010
    101010 (...)"
    );

    let mut total_rows = String::new();
    println!("Enter total number of rows: ");
    io::stdin().read_line(&mut total_rows).unwrap();
    let total_rows: i32 = total_rows.trim().parse().unwrap();

    for row in 1..=total_rows {
        let mut value = if row % 2 != 0 { 0 } else { 1 };
        for _ in 1..=row {
            print!("{}", value);
            value = 1 - value;
        }
        println!();
    }
}

fn unique_numbers() {
    println!("############# Find unique number in sequence");
    let mut total_num = String::new();
    println!("Enter total numbers: ");
    io::stdin().read_line(&mut total_num).unwrap();
    let total_num: i32 = total_num.trim().parse().unwrap();

    let mut ans = 0;
    for n in 0..total_num {
        let mut num = String::new();
        println!("Enter number {}", n + 1);
        io::stdin().read_line(&mut num).unwrap();
        let num: i32 = num.trim().parse().unwrap();
        ans ^= num; // Bitwise XOR operator
    }

    println!("Unique No. is : {}", ans);
}

pub trait Fizzy {
    fn fizzy(&self) -> String;
}

impl Fizzy for i32 {
    fn fizzy(&self) -> String {
        match (self % 3, self % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => format!("{}", self),
        }
    }
}

fn fizz_buzz() {
    println!("############# FizzBuzz 1 to 50");
    for x in 1..=50 {
        println!("{}", x.fizzy())
    }
}
