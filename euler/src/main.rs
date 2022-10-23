use std::time::SystemTime;

mod euler;

fn timeit<F: Fn() -> T, T>(what: &str, f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("\t{} took {} msecs", what, duration.as_millis());
    result
  }
fn main() {
    println!("Euler problems");
    timeit("p001", || euler::p001::do_it());
    timeit("p002", || euler::p002::do_it());
    timeit("p003", || euler::p003::do_it());
    timeit("p004", || euler::p004::do_it());
    timeit("p005", || euler::p005::do_it());
    timeit("p006", || euler::p006::do_it());
}
