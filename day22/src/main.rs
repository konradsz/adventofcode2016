use std::fs;

const HEIGHT: usize = 31;
const WIDTH: usize = 32;

struct Node {
    size: usize,
    used: usize,
}

fn part_1(grid: &[Node]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(index_a, node_a)| {
            grid.iter()
                .enumerate()
                .filter(|(index_b, node_b)| {
                    let a_used = node_a.used;
                    let b_avail = node_b.size - node_b.used;

                    index_a != *index_b && a_used != 0 && b_avail >= a_used
                })
                .count()
        })
        .sum::<usize>()
}

fn print(grid: &[Node]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let node = &grid[x * HEIGHT + y];
            if node.size > 500 {
                print!("# ");
            }
            else if node.used == 0 {
                print!("_ ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let mut grid = Vec::new();
    for line in input.lines().skip(2) {
        let mut props = line
            .split_whitespace()
            .skip(1)
            .map(|p| p.trim_matches('T').parse::<usize>().unwrap());
        let size = props.next().unwrap();
        let used = props.next().unwrap();
        grid.push(Node { size, used });
    }

    assert_eq!(985, part_1(&grid));
}
