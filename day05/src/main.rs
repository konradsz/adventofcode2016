use std::collections::BTreeMap;

fn part_1(key: &str) -> String {
    (0..)
        .map(|index| get_first_32_bits_of_md5(key, index))
        .filter(|number| number & 0xFFFF_F000 == 0)
        .take(8)
        .map(|number| std::char::from_digit(number >> 8, 16).unwrap())
        .collect::<String>()
}

fn part_2(key: &str) -> String {
    let mut letters = BTreeMap::new();

    let mut mask = 0b0000_0000;
    let mut current_index = 0;

    while mask != 0b1111_1111 {
        let (index, position, number) = (current_index..)
            .map(|index| (index, get_first_32_bits_of_md5(key, index)))
            .filter(|(_, number)| number & 0xFFFF_F000 == 0)
            .filter(|(_, number)| (number >> 8) < 8u32)
            .filter(|(_, number)| mask & (1 << (number >> 8)) == 0)
            .map(|(index, number)| (index, number >> 8, (number & 0x0F0) >> 4))
            .next()
            .unwrap();

        mask |= 1 << position;
        current_index = index;
        letters.insert(position, number);
    }

    letters
        .values()
        .map(|number| std::char::from_digit(*number, 16).unwrap())
        .collect::<String>()
}

fn get_first_32_bits_of_md5(key: &str, index: usize) -> u32 {
    let digest = *md5::compute(format!("{}{}", key, index));
    ((digest[0] as u32) << 24)
        + ((digest[1] as u32) << 16)
        + ((digest[2] as u32) << 8)
        + digest[3] as u32
}

fn main() {
    const KEY: &str = "cxdnnyjw";

    assert_eq!("f77a0e6e", part_1(KEY));
    assert_eq!("999828ec", part_2(KEY));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!("18f47a30", part_1("abc"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("05ace8e3", part_2("abc"));
    }
}
