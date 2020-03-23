#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

type Row = Vec<Tile>;

fn part_1(input: &str) -> usize {
    const ROWS: usize = 40;
    let first_row = parse_row(input);
    count_safe_tiles_in_map(first_row, ROWS)
}

fn part_2(input: &str) -> usize {
    const ROWS: usize = 400_000;
    let first_row = parse_row(input);
    count_safe_tiles_in_map(first_row, ROWS)
}

fn parse_row(input: &str) -> Row {
    input
        .chars()
        .map(|c| match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("unknown tile type!"),
        })
        .collect()
}

fn count_safe_tiles_in_map(first_row: Row, rows: usize) -> usize {
    let mut map = Vec::with_capacity(rows);
    map.push(first_row);

    for previous_row_index in 0..rows - 1 {
        let mut row = Vec::new();
        for index in 0..map[previous_row_index].len() {
            let previous_row = &map[previous_row_index];
            let lcr = (
                get_left(index, previous_row),
                get_center(index, previous_row),
                get_right(index, previous_row),
            );

            if lcr == (Tile::Trap, Tile::Trap, Tile::Safe)
                || lcr == (Tile::Safe, Tile::Trap, Tile::Trap)
                || lcr == (Tile::Trap, Tile::Safe, Tile::Safe)
                || lcr == (Tile::Safe, Tile::Safe, Tile::Trap)
            {
                row.push(Tile::Trap);
            } else {
                row.push(Tile::Safe);
            }
        }
        map.push(row);
    }

    map.iter()
        .flatten()
        .filter(|tile| **tile == Tile::Safe)
        .count()
}

fn get_left(index: usize, row: &Row) -> Tile {
    if index == 0 {
        Tile::Safe
    } else {
        row[index - 1]
    }
}

fn get_center(index: usize, row: &Row) -> Tile {
    row[index]
}

fn get_right(index: usize, row: &Row) -> Tile {
    if index == row.len() - 1 {
        Tile::Safe
    } else {
        row[index + 1]
    }
}

fn main() {
    const INPUT: &str = "......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^..";

    assert_eq!(1_963, part_1(INPUT));
    assert_eq!(20_009_568, part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_safe_tiles_in_map() {
        const INPUT: &str = ".^^.^.^^^^";
        let first_row = parse_row(INPUT);
        assert_eq!(38, count_safe_tiles_in_map(first_row, 10));
    }
}
