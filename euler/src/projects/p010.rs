use primes::{PrimeSet, Sieve};

pub fn do_it() {
    let mut pset = Sieve::new();
    let mut res: u64 = 0;

    for (_, n) in pset.iter().enumerate() {
        if n < 2000000 {
            res += n;
        } else {
            break;
        }
    }
    println!("p010=>{}", res);
}
