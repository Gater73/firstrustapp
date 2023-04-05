use std::io;

fn main() {
    let mut buf = String::new();
    let mut mode = String::new();
    let mut shiftByString = String::new();
    println!("You want to cipher or dechiper? (c or d) ");
    io::stdin().read_line(&mut mode).unwrap();
    mode = mode.trim().to_string();
    println!("Type what you need to cipher:");
    io::stdin().read_line(&mut buf).expect("failed to read line");
    buf = buf.trim().to_string();
    println!("How much to shift? (can not use negative values!):");
    io::stdin().read_line(&mut shiftByString).expect("failed to read line");
    let shiftBy: i16 = shiftByString.trim().parse().unwrap();
    if mode == "c" {
        println!("Ciphered string: {}", caesar_cipher(buf, shiftBy));
    } else {
        println!("Deciphered string: {}", caesar_decipher(buf, shiftBy));
    };
    
}

fn caesar_cipher(input: String, shift: i16) -> String {
    let mut output = String::new();
    let shift = shift % 26; // Ensure shift is within 0-25 range
    for c in input.chars() {
        let shifted = match c {
            'a'..='z' => ((c as u8 - b'a' + shift as u8) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' + shift as u8) % 26 + b'A') as char,
            _ => c,
        };
        output.push(shifted);
    }
    output
}

fn caesar_decipher(input: String, shift: i16) -> String {
    let mut output = String::new();
    let shift = shift % 26; // Ensure shift is within 0-25 range
    for c in input.chars() {
        let deciphered = match c {
            'a'..='z' => ((c as u8 - b'a' + 26 - shift as u8) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' + 26 - shift as u8) % 26 + b'A') as char,
            _ => c,
        };
        output.push(deciphered);
    }
    output
}