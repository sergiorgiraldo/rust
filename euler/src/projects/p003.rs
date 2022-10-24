fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };

    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    x
}

fn factors(x: u64) -> Vec<u64> {
    if x <= 1 {
        return vec![];
    };
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = x;
    loop {
        let m = firstfac(curn);
        lst.push(m);
        if m == curn {
            break;
        } else {
            curn /= m
        };
    }
    lst
}

pub fn do_it() {
    let nb: u64 = 600851475143;
    let factors = factors(nb);

    println!("p003=>{}", factors[factors.len() - 1]);
}
