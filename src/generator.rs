use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashSet;

pub fn generate_password(length: usize, allow: String) -> Result<String, &'static str> {
    let valid_chars: HashSet<char> = ['a', 'A', 'n', 's'].iter().cloned().collect();
    let allow_set: HashSet<char> = allow.chars().collect();

    // Validate 'allow' argument
    if !allow_set.is_subset(&valid_chars) {
        return Err("Invalid 'allow' argument. Use only 'a', 'A', 'n', or 's'.");
    }

    let mut charset = Vec::new();
    let mut required_chars = Vec::new();
    let mut rng = thread_rng();

    // Character pools
    let lowercase: Vec<u8> = (b'a'..=b'z').collect();
    let uppercase: Vec<u8> = (b'A'..=b'Z').collect();
    let numbers: Vec<u8> = (b'0'..=b'9').collect();
    let special_chars: Vec<u8> = b"!@#$%^&*()-_=+[]{}|;:',.<>?/~".to_vec();

    if allow_set.contains(&'a') {
        charset.extend(&lowercase);
        required_chars.push(*lowercase.choose(&mut rng).unwrap());
    }
    if allow_set.contains(&'A') {
        charset.extend(&uppercase);
        required_chars.push(*uppercase.choose(&mut rng).unwrap());
    }
    if allow_set.contains(&'n') {
        charset.extend(&numbers);
        required_chars.push(*numbers.choose(&mut rng).unwrap());
    }
    if allow_set.contains(&'s') {
        charset.extend(&special_chars);
        required_chars.push(*special_chars.choose(&mut rng).unwrap());
    }

    if charset.is_empty() {
        return Err("No valid characters selected for password generation.");
    }

    // Fill the reset of the password with random characters
    let mut password: Vec<u8> = (0..length - required_chars.len())
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    password.extend(required_chars);

    password.shuffle(&mut rng);

    Ok(password.into_iter().map(|c| c as char).collect())
}
