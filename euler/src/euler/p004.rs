fn is_palindrome(nb:i64) -> bool{
    return nb.to_string().chars().rev().collect::<String>() == nb.to_string();
}

pub fn do_it(){
    let mut largest = 0;
    for i in (100..=999).rev(){
        for j in (100..=999).rev(){        
            let prod = i * j;
            if prod > largest && is_palindrome(prod){
                largest = prod;
                break;
            }
            else if largest > prod {
                break;
            }
        }
    }
    println!("p004=>{}",largest);
}