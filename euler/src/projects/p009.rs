pub fn do_it() {
    let sum = 1000;
    let mut target = 0;
    'outer: for a in 1..sum {
        for b in a..(sum - a) {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                target = a * b * c;
                break 'outer;
            }
        }
    }
    println!("p009=>{}", target);
}
