use std::fs;

fn part_1(banned_ranges: &[Range]) -> usize {
    banned_ranges[0].max + 1
}

fn part_2(banned_ranges: &[Range]) -> usize {
    banned_ranges
        .windows(2)
        .map(|window| window[1].min - window[0].max - 1)
        .sum::<usize>()
}

#[derive(Copy, Clone)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    fn try_to_merge(&mut self, other: &Range) -> bool {
        if !self.can_be_merged(other) {
            return false;
        }

        self.merge(other);

        true
    }

    fn can_be_merged(&self, other: &Range) -> bool {
        (other.min > self.min && other.min <= self.max)
            || (other.min == self.max + 1)
            || (other.max >= self.min && other.max <= self.max)
            || (other.max == self.min.saturating_sub(1))
    }

    fn merge(&mut self, other: &Range) {
        if other.min < self.min {
            self.min = other.min;
        }

        if other.max > self.max {
            self.max = other.max;
        }
    }
}

fn merge_overlapping(ranges: &[Range]) -> Vec<Range> {
    let mut merged_ranges = Vec::new();
    for range in ranges.iter() {
        if merged_ranges
            .iter_mut()
            .any(|banned: &mut Range| banned.try_to_merge(range))
        {
            continue;
        } else {
            merged_ranges.push(*range);
        }
    }

    merged_ranges
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let mut banned_ranges = Vec::new();
    for line in input.lines() {
        let mut range = line.split('-');
        let min = range.next().unwrap().parse::<usize>().unwrap();
        let max = range.next().unwrap().parse::<usize>().unwrap();

        banned_ranges.push(Range::new(min, max));
    }
    banned_ranges.sort_by(|lhs, rhs| lhs.min.partial_cmp(&rhs.min).unwrap());

    while banned_ranges
        .windows(2)
        .any(|window| window[0].can_be_merged(&window[1]))
    {
        banned_ranges = merge_overlapping(&banned_ranges);
    }

    assert_eq!(19_449_262, part_1(&banned_ranges));
    assert_eq!(119, part_2(&banned_ranges));
}
