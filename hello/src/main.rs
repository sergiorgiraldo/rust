use hello::greet;
use rand::thread_rng;
use rand::RngCore;
fn main() {
    greet("Sergio");
    let mut data = [0u8; 8];
    thread_rng().fill_bytes(&mut data);
    println!("{:?}", data)
}
