use rand::Rng;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;
use zxcvbn::time_estimates::CrackTimes;

fn main() {
    // Get user inputs
    let char_count = get_input("How many characters should the password contain?");
    let include_num = get_bool_input("Should the password contain numbers? (y/n)");
    let spec_count = get_input("How many special characters should the password contain?");

    // Generate random password
    let password = generate_password(char_count, include_num, spec_count);

    // Score the password using zxcvbn
    let estimate = zxcvbn(&password, &[]).unwrap();
    let score = estimate.score();
    let guesses = estimate.crack_times().guesses();
    let time2crack = estimate.crack_times().offline_slow_hashing_1e4_per_second().to_string();

    // Output password and score
    println!("Generated password: {}", password);
    println!("Password strength score: {}", score);
    println!("Estimated number of guesses: {}", guesses);
    println!("ERstimated time to crack: {}", time2crack);
}

// Helper function to get integer input from user
fn get_input(prompt: &str) -> usize {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

// Helper function to get boolean input from user
fn get_bool_input(prompt: &str) -> bool {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.to_lowercase().starts_with('y')
}

// Function to generate random password based on user inputs
fn generate_password(char_count: usize, include_num: bool, mut spec_count: usize) -> String {
    let mut password = String::new();
    let chars: Vec<char> = if include_num {
        (0..=9).map(|i| char::from_digit(i, 10).unwrap()).collect()
    } else {
        vec![]
    };
    let special_chars = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '='];
    let mut rng = rand::thread_rng();

    for _ in 0..char_count {
        if !chars.is_empty() && rng.gen_range(0..2) == 1 {
            password.push(*chars.choose(&mut rng).unwrap());
        } else if spec_count > 0 && rng.gen_range(0..2) == 1 {
            password.push(*special_chars.choose(&mut rng).unwrap());
            spec_count -= 1;
        } else {
            password.push(rng.gen_range(97..123) as u8 as char);
        }
    }

    password
}
