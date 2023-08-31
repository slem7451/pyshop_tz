pub mod hash_finder {
    use sha2::{Sha256, Digest};
    fn get_hash(num: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(num.to_string());
        format!("{:x}", hasher.finalize())
    }

    fn check_zeroes(hash: &String, zeroes_count: u32) -> bool {
        let mut count = 0;
        for i in 1..=zeroes_count {
            if hash.chars().nth(hash.len() - i as usize).unwrap() == '0' {
                count += 1;
            }
        }
        count == zeroes_count
    }

    pub fn find_hash_with_zeroes(n: u32, mut f: u32) {
        let mut num: u64 = 1;
        while f > 0 {
            let hash = get_hash(num);
            if check_zeroes(&hash, n) {
                println!("{}, \"{}\"", num, hash);
                f -= 1;
            }
            num += 1;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{check_zeroes, get_hash};

        #[test]
        fn check_hash() {
            assert_eq!(get_hash(5225), "0d0fa5f5d36a2fc2c5d61338f6c5ba3c7dc2e6bda4a47f5b04cc807ba79d7d12");
            assert_eq!(get_hash(52), "41cfc0d1f2d127b04555b7246d84019b4d27710a3f3aff6e7764375b1e06e05d");
            assert_eq!(get_hash(1), "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b");
        }

        #[test]
        fn check_zeroes_answer() {
            assert_eq!(check_zeroes(&get_hash(31214), 4), true);
            assert_eq!(check_zeroes(&get_hash(31214), 3), true);
            assert_eq!(check_zeroes(&get_hash(31214), 5), false);
            assert_eq!(check_zeroes(&get_hash(1), 0), true);
            assert_eq!(check_zeroes(&get_hash(1), 1), false);
        }
    }
}