use num::bigint::BigInt;
use num_traits::One;

pub fn do_it() {
    let mut two_power_1000: BigInt = One::one();
    let res: i32;
    for _ in 1..=1000 {
        two_power_1000 = two_power_1000 * 2;
    }
    res = two_power_1000
        .to_string()
        .chars()
        .map(|c| (c.to_string()).parse::<i32>().unwrap())
        .sum();
    println!("p016=>{}", res);
}
