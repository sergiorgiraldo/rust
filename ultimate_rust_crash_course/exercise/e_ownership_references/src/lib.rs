pub fn inspect(s: &String){
    if s.ends_with("s"){
        println!("plural");
    }
    else{
        println!("singular");
    }
}

pub fn change(s: &mut String) -> String{
    if ! s.ends_with("s"){
        s.push_str("s");
    }

    s.to_string()
}

pub fn eat(s: String) -> bool{
    if s.starts_with("b") && s.contains("a"){
        true
    }
    else{
        false
    }
}

pub fn bedazzle(s: &mut String){
    *s = "sparkly".to_string();
}