use primes::{Sieve, PrimeSet};

pub fn do_it() {
    let mut pset = Sieve::new();
    let mut res = 0;
    
    for (_, n) in pset.iter().enumerate().take(10001) {
        res =n;
    }
    println!("p007=>{}", res);
}
