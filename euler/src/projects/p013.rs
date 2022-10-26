use num::bigint::BigInt;

pub fn do_it(){
    let filename = "./src/data/013.txt";

    let contents = euler::get_contents(filename).unwrap();

    let arr = contents
        .split("\n")
        .map(|num| num.parse::<BigInt>().unwrap())
        .collect::<Vec<BigInt>>();
    let string_sum = format!("{}", arr.iter().sum::<BigInt>());
    println!("p013=>{}", &string_sum[..10]); 
}