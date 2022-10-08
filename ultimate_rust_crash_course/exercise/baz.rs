use std::fs::File;

fn main(){
    let res = File::open("foo");
    match res{
        Ok(_f) => { println!("file opened"); }
        Err(_e) =>  { println!("file not found"); }
    }
}
