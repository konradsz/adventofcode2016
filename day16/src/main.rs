const INITIAL_STATE: &str = "10011111011011001";

fn part_1() -> String {
    const DISK_LENGTH: usize = 272;
    calculate_checksum_for_disk_length(INITIAL_STATE, DISK_LENGTH)
}

fn part_2() -> String {
    const DISK_LENGTH: usize = 35_651_584;
    calculate_checksum_for_disk_length(INITIAL_STATE, DISK_LENGTH)
}

fn calculate_checksum_for_disk_length(input: &str, disk_length: usize) -> String {
    let data = generate_data_to_fill_disk(input, disk_length);
    calculate_checksum(&data)
}

fn calculate_checksum(input: &str) -> String {
    let mut checksum = String::new();
    input.as_bytes().chunks(2).for_each(|w| {
        if w[0] == w[1] {
            checksum.push('1');
        } else {
            checksum.push('0');
        }
    });

    while checksum.len() % 2 == 0 {
        checksum = calculate_checksum(&checksum);
    }

    checksum
}

fn generate_data_to_fill_disk(input: &str, disk_length: usize) -> String {
    let mut output = String::from(input);

    while output.len() < disk_length {
        output = generate_data(&output);
    }

    output.split_off(disk_length);

    output
}

fn generate_data(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    let reversed_negated: String = reversed
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("unexpected character"),
        })
        .collect();

    input.to_owned() + "0" + &reversed_negated
}

fn main() {
    assert_eq!("10111110010110110", part_1());
    assert_eq!("01101100001100100", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_checksum_for_disk_length() {
        assert_eq!("01100", calculate_checksum_for_disk_length("10000", 20));
    }

    #[test]
    fn test_calculate_checksum() {
        assert_eq!("100", calculate_checksum("110010110100"));
    }

    #[test]
    fn test_generate_data_to_fill_disk() {
        assert_eq!(
            "10000011110010000111",
            generate_data_to_fill_disk("10000", 20)
        );
    }

    #[test]
    fn test_generate_data() {
        assert_eq!("100", generate_data("1"));
        assert_eq!("001", generate_data("0"));
        assert_eq!("11111000000", generate_data("11111"));
        assert_eq!("1111000010100101011110000", generate_data("111100001010"));
    }
}
