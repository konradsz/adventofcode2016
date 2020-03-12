use std::collections::HashMap;
use std::fs;

type Occurrences = HashMap<char, usize>;

fn part_1(indexed_occurrences: &[Occurrences]) -> String {
    indexed_occurrences
        .iter()
        .map(|letters| {
            letters
                .iter()
                .max_by(|lhs, rhs| lhs.1.cmp(rhs.1))
                .unwrap()
                .0
        })
        .collect::<String>()
}

fn part_2(indexed_occurrences: &[Occurrences]) -> String {
    indexed_occurrences
        .iter()
        .map(|letters| {
            letters
                .iter()
                .min_by(|lhs, rhs| lhs.1.cmp(rhs.1))
                .unwrap()
                .0
        })
        .collect::<String>()
}

fn count_occurences(input: &str) -> Vec<Occurrences> {
    let mut indexed_occurrences = Vec::new();
    for line in input.lines() {
        for (index, c) in line.chars().enumerate() {
            if indexed_occurrences.len() <= index {
                indexed_occurrences.push(Occurrences::new());
            }

            if let Some(occurence) = indexed_occurrences.get_mut(index) {
                let counter = (*occurence).entry(c).or_insert(0);
                *counter += 1;
            }
        }
    }

    indexed_occurrences
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let indexed_occurrences = count_occurences(input);
    assert_eq!("umejzgdw", part_1(&indexed_occurrences));
    assert_eq!("aovueakv", part_2(&indexed_occurrences));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "eedadn\n\
                         drvtee\n\
                         eandsr\n\
                         raavrd\n\
                         atevrs\n\
                         tsrnev\n\
                         sdttsa\n\
                         rasrtv\n\
                         nssdts\n\
                         ntnada\n\
                         svetve\n\
                         tesnvt\n\
                         vntsnd\n\
                         vrdear\n\
                         dvrsen\n\
                         enarar\n";

    #[test]
    fn test_part_1() {
        let indexed_occurrences = count_occurences(INPUT);
        assert_eq!("easter", part_1(&indexed_occurrences));
    }

    #[test]
    fn test_part_2() {
        let indexed_occurrences = count_occurences(INPUT);
        assert_eq!("advent", part_2(&indexed_occurrences));
    }
}
