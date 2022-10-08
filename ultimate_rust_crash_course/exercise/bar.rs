fn main(){
    let mut x = None;
    println!("x is none");
    if x.is_some() { println!("is some"); }
    if x.is_none() { println!("is none"); }
    x = Some(5);
    println!("x is 5 now");
    if x.is_some() { println!("is some"); }
    if x.is_none() { println!("is none"); }
    //x += 1;
}
