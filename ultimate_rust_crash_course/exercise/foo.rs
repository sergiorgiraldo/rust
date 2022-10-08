fn main(){
    let mut s1 = String::from("abc");
    println!("{}", s1);
    d(&mut s1);
    println!("{}", s1);

    fn d(s: &mut String) {
        s.insert_str(0, "Hi, ");
    }
}
