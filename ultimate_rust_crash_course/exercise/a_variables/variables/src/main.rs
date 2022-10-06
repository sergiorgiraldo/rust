const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 =2;

fn main() {
    let (mut missiles, ready):(i32, i32) = (STARTING_MISSILES,READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    let (missiles2, ready2):(i32, i32) = (STARTING_MISSILES,READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready2, missiles2);
    println!("{} missiles left", missiles2 - ready2);

    let _dummy = "foo"; //w/out the underscore gives me the warning "warning: unused variable: `dummy`"
    //READY_AMOUNT=1; //error: cannot assign to this expression
}
