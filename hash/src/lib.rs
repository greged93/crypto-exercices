use hex_literal::hex;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha512};
use std::collections::HashMap;

fn sha_512_n(input: &[u8], n: usize) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(input);
    hasher.finalize()[..n].to_vec()
}

fn birthday_attack(n: usize) -> u32 {
    let mut map: HashMap<Vec<u8>, bool> = HashMap::new();
    let mut num = 0u32;
    loop {
        let s = get_random_input();
        let hash = sha_512_n(&s, n);
        if map.contains_key(&hash) {
            break num;
        }
        map.insert(hash, true);
        num += 1;
    }
}

fn target(target: &[u8], n: usize) -> u32 {
    let mut num = 0u32;
    loop {
        let s = get_random_input();
        let hash = sha_512_n(&s, n);
        if target.eq(&hash) {
            break num;
        }
        num += 1;
    }
}

fn get_random_input() -> Vec<u8> {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha_512_n() {
        assert_eq!(hex!("4b3d70af9acffb7b"), sha_512_n(b"hey dude", 8)[..]);
    }

    #[test]
    fn test_sha_512_1() {
        let mut timing: Vec<u128> = vec![];
        for _ in 0..100 {
            let instant = std::time::Instant::now();
            birthday_attack(1);
            let time_elapsed = instant.elapsed();
            timing.push(time_elapsed.as_nanos());
        }
        let mean: u128 = timing.into_iter().sum();
        println!("mean timing is {}", mean / 100);
    }

    #[test]
    fn test_sha_512_2() {
        let mut timing: Vec<u128> = vec![];
        for _ in 0..100 {
            let instant = std::time::Instant::now();
            birthday_attack(2);
            let time_elapsed = instant.elapsed();
            timing.push(time_elapsed.as_millis());
        }
        let mean: u128 = timing.into_iter().sum();
        println!("mean timing is {}", mean / 100);
    }

    #[test]
    fn test_target() {
        let mut timing: Vec<u128> = vec![];
        let t = hex!("3d4b");
        for _ in 0..5 {
            let instant = std::time::Instant::now();
            target(&t, 2);
            let time_elapsed = instant.elapsed();
            timing.push(time_elapsed.as_millis());
        }
        let mean: u128 = timing.into_iter().sum();
        println!("mean timing is {}", mean / 5);
    }
}
