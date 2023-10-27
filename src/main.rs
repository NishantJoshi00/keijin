fn main() {
    generate_aes256_key();
}


pub fn generate_aes256_key() -> [u8; 32] {
    use ring::rand::SecureRandom;

    let rng = ring::rand::SystemRandom::new();
    let mut key: [u8; 256 / 8] = [0_u8; 256 / 8];
    rng.fill(&mut key).unwrap();
    print!("{}", hex::encode(key));
    key
}
