use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut password = String::new();

    if args.len() == 1 {
        print!("Enter a password: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut password).unwrap();
    } else if args.len() == 2 {
        password = args[1].clone();
    } else {
        println!("Usage: password-strength [password]");
        process::exit(1);
    }

    let password = password.trim();

    if password.is_empty() {
        println!("Password cannot be empty.");
        process::exit(1);
    }

    let dictionary_path = Path::new("data/dictionary.txt");
    let common_passwords_path = Path::new("data/common-passwords.txt");

    if is_password_in_dictionary(password, dictionary_path) || is_password_common(password, common_passwords_path) {
        println!("Password is weak: It matches a common password or dictionary word.");
    } else {
        let entropy = calculate_entropy(password);
        println!("Password strength: {}", get_strength(entropy));
    }
}

fn is_password_in_dictionary(password: &str, dictionary_path: &Path) -> bool {
    let dictionary = fs::read_to_string(dictionary_path).unwrap();
    dictionary.lines().any(|line| line == password)
}

fn is_password_common(password: &str, common_passwords_path: &Path) -> bool {
    let common_passwords = fs::read_to_string(common_passwords_path).unwrap();
    common_passwords.lines().any(|line| line == password)
}

fn calculate_entropy(password: &str) -> f64 {
    let length = password.chars().count();
    let range = password.chars().collect::<std::collections::HashSet<char>>().len();
    length as f64 * (range as f64).log2()
}

fn get_strength(entropy: f64) -> &'static str {
    if entropy < 28.0 {
        "Very weak"
    } else if entropy < 36.0 {
        "Weak"
    } else if entropy < 60.0 {
        "Reasonable"
    } else if entropy < 128.0 {
        "Strong"
    } else {
        "Very strong"
    }
}
