use num::bigint::BigInt;

pub fn do_it() {
    let filename = "./src/data/013.txt";
    let contents = euler::get_contents(filename).unwrap();

    let arr = contents
        .split('\n')
        .map(|num| num.parse::<BigInt>().unwrap())
        .collect::<Vec<BigInt>>();
    let sum = arr.iter().sum::<BigInt>();

    println!("p013=>{}", &sum.to_string()[..10]);
}
