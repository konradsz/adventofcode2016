use itertools::Itertools;
use std::fs;
use regex::Regex;

type Entry = (usize, usize, usize);

fn part_1(data: &[Entry]) -> usize {
    data.iter().filter(|entry| is_valid_triangle(entry)).count()
}

fn part_2(data: &[Entry]) -> usize {
    let grouped_entries: Vec<[Entry; 3]> = data
        .iter()
        .tuples::<(_, _, _)>()
        .map(|entries_group| {
            [
                ((entries_group.0).0, (entries_group.1).0, (entries_group.2).0),
                ((entries_group.0).1, (entries_group.1).1, (entries_group.2).1),
                ((entries_group.0).2, (entries_group.1).2, (entries_group.2).2),
            ]
        })
        .collect();

    grouped_entries
        .iter()
        .flatten()
        .filter(|entry| is_valid_triangle(entry))
        .count()
}

fn is_valid_triangle(entry: &Entry) -> bool {
    entry.0 + entry.1 > entry.2 && entry.0 + entry.2 > entry.1 && entry.1 + entry.2 > entry.0
}

fn parse_input(input: &str) -> Vec<Entry> {
    let re = Regex::new(r"^\s*(?P<dim_1>\d+)\s*(?P<dim_2>\d+)\s*(?P<dim_3>\d+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(&line).unwrap();
            (
                caps["dim_1"].parse::<usize>().unwrap(),
                caps["dim_2"].parse::<usize>().unwrap(),
                caps["dim_3"].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let entries = parse_input(input);

    assert_eq!(862, part_1(&entries));
    assert_eq!(1577, part_2(&entries));
}
