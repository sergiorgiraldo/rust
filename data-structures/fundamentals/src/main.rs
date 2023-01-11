use std::io;
fn main() {
    println!("########## fahrenheit to celsius");
    let mut fahren_value = 0;
    let mut celsius_value;

    while fahren_value <= 200 {
        celsius_value = (5 * (fahren_value - 32)) / 9;
        println!("{}  {}", fahren_value, celsius_value);
        fahren_value += 20;
    }

    println!("########## Find the minimum and maximum of N Numbers");
    let mut total_num = String::new();
    println!("Enter total numbers: ");
    io::stdin().read_line(&mut total_num).unwrap();
    let total_num: u32 = total_num.trim().parse().unwrap();

    let mut min_so_far = std::i32::MAX;
    let mut max_so_far = std::i32::MIN;
    
    for _i in 0..total_num {
        println!("Enter number {}: ", _i + 1);
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num_int: i32 = num.trim().parse().unwrap();
        if num_int < min_so_far {
            min_so_far = num_int;
        }
        if num_int > max_so_far {
            max_so_far = num_int;
        }
    }
    println!("Minimum Number: {}", min_so_far);
    println!("Maximum Number: {}", max_so_far);

    println!("########## Print number pyramid for N rows");
    let mut total_rows = String::new();
    println!("Enter total number of rows: ");
    io::stdin().read_line(&mut total_rows).unwrap();
    let total_rows: u32 = total_rows.trim().parse().unwrap();

    let mut row = 1;
    let mut col;
    let mut value = 0;

    while row <= total_rows {
        col = total_rows - row;
        while col > 0 {
            print!(" ");
            col -= 1;
        }

        col = row;
        while col > 0 {
            value += 1;
            print!("{}", value);
            col -= 1;
        }

        col = row - 1;
        while col > 0 {
            value -= 1;
            print!("{}", value);
            col -= 1;
        }

        println!();
        row += 1;
    }

    println!("########## Find the square root of a number without using any predefined function");
    let mut number = String::new();
    println!("Enter the number: ");
    io::stdin().read_line(&mut number).unwrap();
    let number: f64 = number.trim().parse().unwrap();

    let mut precision = String::new();
    println!("Enter the precision: ");
    io::stdin().read_line(&mut precision).unwrap();
    let mut precision: i32 = precision.trim().parse().unwrap();
    let decimals = 10i64.pow(precision as u32) as f64;
    let mut ans:f64 = 0.0;
    let mut inc:f64 = 1.0;
    const STEP:f64 = 10.0;

    while precision >= 0 {
        while ans * ans <= number {
            ans = ans + inc;
        }
        ans = ans - inc;
        inc = inc / STEP;
        precision -= 1;
    }
    ans = (ans * decimals).round() / decimals;

    println!("{}", ans);
}
