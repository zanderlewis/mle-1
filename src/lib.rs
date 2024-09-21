use rand::Rng;
use std::collections::HashMap;

pub fn encrypt(data: &str, key: &str) -> String {
    let mut encrypted = String::new();
    for (i, c) in data.chars().enumerate() {
        encrypted.push((c as u8 ^ key.as_bytes()[i % key.len()]) as char);
    }
    encrypted
}

pub fn decrypt(encrypted_data: &str, key: &str, dummy_data: &HashMap<String, String>) -> String {
    let mut decrypted = String::new();
    for (i, c) in encrypted_data.chars().enumerate() {
        decrypted.push((c as u8 ^ key.as_bytes()[i % key.len()]) as char);
    }

    if dummy_data.contains_key(&decrypted) {
        dummy_data.get(&decrypted).unwrap().to_string()
    } else {
        decrypted
    }
}

pub fn generate_dummy_data() -> String {
    let mut rng = rand::thread_rng();
    let dummy_length = rng.gen_range(20..101);
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    (0..dummy_length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

#[cfg(test)]
fn main() {
    let key = "supersecretkey";
    let data = "real_data_yaa";

    let encrypted_data = encrypt(data, key);

    let mut dummy_map = HashMap::new();
    for _ in 0..5 {
        let dummy_data = generate_dummy_data();
        dummy_map.insert(encrypt(&dummy_data, key), dummy_data);
    }

    let decrypted_data = decrypt(&encrypted_data, key, &dummy_map);
    println!("Decrypted data with correct key: {}", decrypted_data);

    let wrong_key = "kjwe";
    let decrypted_data_wrong = decrypt(&encrypted_data, wrong_key, &dummy_map);
    println!("Decrypted data with wrong key: {}", decrypted_data_wrong);
}