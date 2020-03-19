use std::collections::VecDeque;

struct Key64thFinder {
    salt: String,
    index: usize,
    hash_cache: VecDeque<String>,
    key_stretching_enabled: bool,
}

impl Key64thFinder {
    fn new(salt: &str, key_stretching_enabled: bool) -> Self {
        let mut cache = VecDeque::with_capacity(1000);
        for index in 0..=1000 {
            let mut digest = format!("{:x}", md5::compute(format!("{}{}", salt, index)));
            if key_stretching_enabled {
                for _ in 0..2016 {
                    digest = format!("{:x}", md5::compute(digest));
                }
            }

            cache.push_back(digest);
        }

        Self {
            salt: String::from(salt),
            index: 0,
            hash_cache: cache,
            key_stretching_enabled,
        }
    }

    fn find_next_potential_key(&mut self) {
        loop {
            let mut digest = format!(
                "{:x}",
                md5::compute(format!("{}{}", self.salt, self.index + 1001))
            );
            if self.key_stretching_enabled {
                for _ in 0..2016 {
                    digest = format!("{:x}", md5::compute(digest));
                }
            }
            self.hash_cache.push_back(digest);

            let hash = self.hash_cache.front().unwrap();
            if hash
                .as_bytes()
                .windows(3)
                .any(|w| w[0] == w[1] && w[1] == w[2])
            {
                self.index += 1;
                break;
            }

            self.hash_cache.pop_front();

            self.index += 1;
        }
    }

    fn is_key(&mut self) -> bool {
        let hash = self.hash_cache.pop_front().unwrap();
        let c = hash
            .as_bytes()
            .windows(3)
            .find(|w| w[0] == w[1] && w[1] == w[2])
            .map(|w| w[0])
            .unwrap();

        for cached in self.hash_cache.iter() {
            if cached
                .as_bytes()
                .windows(5)
                .any(|w| w[0] == c && w[1] == c && w[2] == c && w[3] == c && w[4] == c)
            {
                return true;
            }
        }

        false
    }

    fn find_index_of_64th_key(&mut self) -> usize {
        let mut count = 0;

        while count != 64 {
            self.find_next_potential_key();
            if self.is_key() {
                println!("Found #{} at index {}", count + 1, self.index - 1);
                count += 1;
            }
        }

        self.index - 1
    }
}

fn main() {
    const KEY: &str = "ngcjuoqr";

    let mut part_1 = Key64thFinder::new(KEY, false);
    assert_eq!(18_626, part_1.find_index_of_64th_key());

    let mut part_2 = Key64thFinder::new(KEY, true);
    assert_eq!(20_092, part_2.find_index_of_64th_key());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut part_1 = Key64thFinder::new("abc", false);
    	assert_eq!(22_728, part_1.find_index_of_64th_key());
    }

    #[test]
    fn test_part_2() {
        let mut part_2 = Key64thFinder::new("abc", true);
    	assert_eq!(22_551, part_2.find_index_of_64th_key());
    }
}
