extern crate zxcvbn;
use std::io;
use zxcvbn::zxcvbn;
use zxcvbn::time_estimates::CrackTimes;

fn main() {
    println!("Enter the password:");
    let mut p = String::new();
 
    io::stdin().read_line(&mut p).expect("failed to readline");

    let estimate = zxcvbn(&p, &[]).unwrap();
    println!("Score (1-4) {}", estimate.score()); 
    println!("Guesses needed {}", estimate.crack_times().guesses()); 
    println!("Estimated time, 10 guesses per second  {}", estimate.crack_times().online_no_throttling_10_per_second()); 
}