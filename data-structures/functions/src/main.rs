fn main() {
    println!("######## Fibonacci");
    println!("fib(1) => {}", fibonacci(1));
    println!("fib(2) => {}", fibonacci(2));
    println!("fib(3) => {}", fibonacci(3));
    println!("fib(4) => {}", fibonacci(4));
    println!("fib(5) => {}", fibonacci(5));

    println!("nCr(4,2) => {}", n_cr(4, 2));
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => panic!("zero is not a right argument to fibonacci()!"),
        1 | 2 => 1,
        /*
        50    => 12586269025,
        */
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn factorial(num: i32) -> i32 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn n_cr(n: i32, r: i32) -> i32 {
    println!("######## find the number of possible combinations that can be obtained by taking a sample of items from a larger set");
    let ans = factorial(n) / (factorial(r) * factorial(n - r));
    ans
}
