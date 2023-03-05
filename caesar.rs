fn caesar_cipher(text: &str, shift: u8) -> String {
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let text = "Hello, world!";
    let shift = 3;
    let ciphered_text = caesar_cipher(text, shift);

    println!("Original text: {}", text);
    println!("Ciphered text: {}", ciphered_text);
}


fn caesar_decipher(text: &str, shift: u8) -> String {
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (c as u8 - base + 26 - shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let text = "Khoor, zruog!";
    let shift = 3;
    let deciphered_text = caesar_decipher(text, shift);

    println!("Original text: {}", text);
    println!("Deciphered text: {}", deciphered_text);
}
