use std::collections::BTreeMap;
use std::fs;

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(is_room_real)
        .map(get_sector_id)
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .filter(is_room_real)
        .map(|entry| (get_sector_id(entry), decrypt(entry)))
        .find(|data| data.1.contains("northpole"))
        .unwrap();
    data.0
}

fn is_room_real(entry: &&str) -> bool {
    let segments: Vec<&str> = entry.split(|c| c == '-' || c == '[' || c == ']').collect();
    let mut letters: BTreeMap<char, usize> = BTreeMap::new();
    segments[0..segments.len() - 3].iter().for_each(|segment| {
        for c in segment.chars() {
            *letters.entry(c).or_insert(0) += 1;
        }
    });

    let mut stats: Vec<(char, usize)> = letters.into_iter().collect();
    stats.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));

    let sorted_letters: String = stats.iter().map(|elem| elem.0).collect();
    sorted_letters.contains(segments[segments.len() - 2])
}

fn get_sector_id(entry: &str) -> usize {
    entry
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn decrypt(entry: &str) -> String {
    let sector_id = get_sector_id(entry);

    let decrypted: String = entry
        .chars()
        .take_while(|c| c.is_alphabetic() || *c == '-')
        .map(|c| {
            if c.is_alphabetic() {
                let current_shift = (c as usize) - 'a' as usize;
                let new_shift = (current_shift + sector_id) % ('z' as usize - 'a' as usize + 1);
                ('a' as usize + new_shift) as u8 as char
            } else {
                ' '
            }
        })
        .collect();

    decrypted
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(245_102, part_1(input));
    assert_eq!(324, part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]\n\
            a-b-c-d-e-f-g-h-987[abcde]\n\
            not-a-real-room-404[oarel]\n\
            totally-real-room-200[decoy]\n";

        assert_eq!(1514, part_1(input));
    }

    #[test]
    fn test_decrypt() {
        let input = "qzmt-zixmtkozy-ivhz-343";
        assert_eq!("very encrypted name", decrypt(input).trim());
    }
}
