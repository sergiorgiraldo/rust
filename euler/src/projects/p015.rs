use num::bigint::{BigInt, ToBigInt};

fn factorial(num: BigInt) -> BigInt {
    if num.eq(&0_i32.to_bigint().unwrap()) || num.eq(&1_i32.to_bigint().unwrap()) {
        1_i32.to_bigint().unwrap()
    } else {
        factorial(&num - 1) * &num
    }
}

///https://towardsdatascience.com/understanding-combinatorics-number-of-paths-on-a-grid-bddf08e28384
pub fn do_it() {
    let res = factorial(40_i32.to_bigint().unwrap())
        / (factorial(20_i32.to_bigint().unwrap()) * factorial(20_i32.to_bigint().unwrap()));
    println!("p015=>{}", res);
}
