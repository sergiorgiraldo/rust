fn make_fib(n: i64) -> Vec<i64> {
    let mut x = vec![1, 1];
    let mut index = 2;
    let mut upper = 1;
    while upper < n {
        let next_x = x[index - 1] + x[index - 2];
        upper = next_x;
        index += 1;
        if upper < n {
            x.push(next_x);
        }
    }
    x
}

pub fn do_it() {
    let mut sum: i64 = 0;
    let fibonacci = make_fib(4000000);
    for i in fibonacci {
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("p002=>{}", sum);
}
