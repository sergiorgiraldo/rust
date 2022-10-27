fn stringfy(i: usize) -> String {
    let ret = match i {
        1 => "One".to_owned(),
        2 => "Two".to_owned(),
        3 => "Three".to_owned(),
        4 => "Four".to_owned(),
        5 => "Five".to_owned(),
        6 => "Six".to_owned(),
        7 => "Seven".to_owned(),
        8 => "Eight".to_owned(),
        9 => "Nine".to_owned(),
        10 => "Ten".to_owned(),
        11 => "Eleven".to_owned(),
        12 => "Twelve".to_owned(),
        13 => "Thirteen".to_owned(),
        14 => "Fourteen".to_owned(),
        15 => "Fifteen".to_owned(),
        16 => "Sixteen".to_owned(),
        17 => "Seventeen".to_owned(),
        18 => "Eighteen".to_owned(),
        19 => "Nineteen".to_owned(),
        20 => "Twenty".to_owned(),
        30 => "Thirty".to_owned(),
        40 => "Forty".to_owned(),
        50 => "Fifty".to_owned(),
        60 => "sixty".to_owned(),
        70 => "Seventy".to_owned(),
        80 => "Eighty".to_owned(),
        90 => "Ninety".to_owned(),
        1000 => "OneThousand".to_owned(),
        _ => {
            let hundreds = i / 100;
            let tens = (i - hundreds * 100) / 10;
            let ones = i - hundreds * 100 - tens * 10;
            let mut ret: String = String::new();
            if hundreds > 0 {
                ret += &(stringfy(hundreds) + "Hundred");
            }
            if tens < 2 {
                let remainder = i - hundreds * 100;
                if remainder != 0 {
                    ret += &("And".to_owned() + &stringfy(i - hundreds * 100));
                }
            } else {
                if hundreds > 0 {
                    ret += "And";
                }
                ret += &stringfy(tens * 10);
                if ones > 0 {
                    ret += &stringfy(ones)
                }
            }
            ret
        }
    };
    ret
}

pub fn do_it() {
    let res = (1..1001)
        .map(|n| stringfy(n))
        .fold(0, |acc, n| acc + n.len());

    println!("p017=>{}", res);
}
