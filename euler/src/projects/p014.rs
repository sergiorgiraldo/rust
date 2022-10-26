pub fn do_it() {
    let mut res = 0;
    let mut max = 0;

    for n in 14..1000000 {
        let mut next = n;
        let mut count = 1;
        while next > 1 {
            match next % 2 {
                0 => {
                    next = even(next);
                    count += 1;
                }
                _ => {
                    next = odd(next);
                    count += 1;
                }
            }
        }
        if (count + 1) > max {
            res = n;
            max = count;
        }
    }
    println!("p014=>{}", res);
}

fn even(n: i64) -> i64 {
    n / 2
}

fn odd(n: i64) -> i64 {
    3 * n + 1
}
