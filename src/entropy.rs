pub fn calculate_entropy(password: &str) -> f64 {
    let charset = calculate_charset(password);
    let length = password.len();

    (length as f64).log2() * charset.log2()
}

fn calculate_charset(password: &str) -> f64 {
    let mut charset = 0u32;

    for byte in password.bytes() {
        if byte >= b'0' && byte <= b'9' {
            charset |= 1;
        } else if byte >= b'a' && byte <= b'z' {
            charset |= 2;
        } else if byte >= b'A' && byte <= b'Z' {
            charset |= 4;
        } else {
            charset |= 8;
        }
    }

    charset.count_ones() as f64
}

pub fn get_strength(entropy: f64) -> String {
    if entropy < 28.0 {
        String::from("Weak")
    } else if entropy < 36.0 {
        String::from("Moderate")
    } else if entropy < 60.0 {
        String::from("Strong")
    } else {
        String::from("Very Strong")
    }
}
