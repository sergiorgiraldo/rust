pub fn do_it() {
    // let mut sum_squares = 0;
    // let mut square_sum = 0;
    // for i in 1..=100{
    //     let square = i * i;
    //     sum_squares += square;
    //     square_sum += i;
    // }
    // println!("p006=>{}", (square_sum * square_sum) - sum_squares);

    let range = 1..=100;
    let sum_squares2: i32 = range.clone().map(|n| n * n).sum();
    let squares_sum2: i32 = range.sum::<i32>().pow(2);
    println!("p006=>{}", squares_sum2 - sum_squares2);
}
