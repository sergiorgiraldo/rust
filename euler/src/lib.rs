use std::time::SystemTime;

pub fn timeit<F: Fn() -> T, T>(what: &str, f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("\t{} took {} msecs", what, duration.as_millis());
    result
}
