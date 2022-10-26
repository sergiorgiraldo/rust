use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

pub fn timeit<F: Fn() -> T, T>(what: &str, f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("\t{} took {} msecs", what, duration.as_millis());
    result
}

pub fn get_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
