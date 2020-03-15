use std::fs;

struct Marker {
    length: usize,
    multiplier: usize,
}

fn decompress_part_1(input: &str) -> usize {
    let mut sum = 0;

    let (head, tail) = read_until_first_marker(input);
    sum += head.len();

    if let Some((marker, rest)) = tail {
        (0..marker.multiplier).for_each(|_| sum += rest[0..marker.length].len());
        sum += decompress_part_1(&rest[marker.length..]);
    }

    sum
}

fn decompress_part_2(input: &str) -> usize {
    let mut sum = 0;

    let (head, tail) = read_until_first_marker(input);
    sum += head.len();

    if let Some((marker, rest)) = tail {
        (0..marker.multiplier).for_each(|_| {
            sum += decompress_part_2(&rest[0..marker.length]);
        });
        sum += decompress_part_2(&rest[marker.length..]);
    }

    sum
}

fn read_until_first_marker(input: &str) -> (&str, Option<(Marker, &str)>) {
    let marker_start_index = input.find('(');

    if let Some(start_index) = marker_start_index {
        let marker_end_index = input.find(')');

        if let Some(end_index) = marker_end_index {
            let marker = &input[start_index + 1..end_index];
            let mut props = marker.split('x').map(|prop| prop.parse::<usize>().unwrap());
            return (
                &input[0..start_index],
                Some((
                    Marker {
                        length: props.next().unwrap(),
                        multiplier: props.next().unwrap(),
                    },
                    &input[end_index + 1..],
                )),
            );
        }
    }

    (input, None)
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(112_830, decompress_part_1(input));
    assert_eq!(10_931_789_799, decompress_part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_part_1() {
        assert_eq!(6, decompress_part_1("ADVENT"));
        assert_eq!(7, decompress_part_1("A(1x5)BC"));
        assert_eq!(9, decompress_part_1("(3x3)XYZ"));
        assert_eq!(11, decompress_part_1("A(2x2)BCD(2x2)EFG"));
        assert_eq!(6, decompress_part_1("(6x1)(1x3)A"));
        assert_eq!(18, decompress_part_1("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn test_decompress_part_2() {
        assert_eq!(9, decompress_part_2("(3x3)XYZ"));
        assert_eq!(20, decompress_part_2("X(8x2)(3x3)ABCY"));
        assert_eq!(
            241_920,
            decompress_part_2("(27x12)(20x12)(13x14)(7x10)(1x12)A")
        );
        assert_eq!(
            445,
            decompress_part_2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
