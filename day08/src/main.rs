#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fs;

type Screen = Vec<Vec<Pixel>>;
const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

#[derive(Copy, Clone, PartialEq)]
enum Pixel {
    On,
    Off,
}

fn part_1(screen: &Screen) -> usize {
    screen.iter().flatten().filter(|p| **p == Pixel::On).count()
}

fn part_2(screen: &Screen) {
    (0..SCREEN_HEIGHT).for_each(|y| {
        (0..SCREEN_WIDTH).for_each(|x| match screen[y][x] {
            Pixel::On => print!("#"),
            Pixel::Off => print!("."),
        });
        println!();
    });
}

fn rect(screen: &mut Screen, width: usize, height: usize) {
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            screen[y][x] = Pixel::On;
        })
    });
}

fn rotate_row(screen: &mut Screen, row: usize, count: usize) {
    let current_row = screen[row].clone();
    (0..SCREEN_WIDTH).for_each(|w| {
        let new_position = (w + count) % SCREEN_WIDTH;
        screen[row][new_position] = current_row[w];
    });
}

fn rotate_column(screen: &mut Screen, column: usize, count: usize) {
    let current_column = (0..SCREEN_HEIGHT)
        .map(|y| screen[y][column])
        .collect::<Vec<Pixel>>();
    (0..SCREEN_HEIGHT).for_each(|h| {
        let new_position = (h + count) % SCREEN_HEIGHT;
        screen[new_position][column] = current_column[h];
    });
}

fn apply_instruction(screen: &mut Screen, instruction: &str) {
    lazy_static! {
        static ref RECT_RE: Regex = Regex::new(r"^rect (?P<width>\d+)x(?P<height>\d+)$").unwrap();
        static ref ROTATE_ROW_RE: Regex =
            Regex::new(r"^rotate row y=(?P<row>\d+) by (?P<count>\d+)$").unwrap();
        static ref ROTATE_COLUMN_RE: Regex =
            Regex::new(r"^rotate column x=(?P<column>\d+) by (?P<count>\d+)$").unwrap();
    }

    if RECT_RE.is_match(instruction) {
        let caps = RECT_RE.captures(instruction).unwrap();
        let width = caps["width"].parse::<usize>().unwrap();
        let height = caps["height"].parse::<usize>().unwrap();
        rect(screen, width, height);
    } else if ROTATE_ROW_RE.is_match(instruction) {
        let caps = ROTATE_ROW_RE.captures(instruction).unwrap();
        let row = caps["row"].parse::<usize>().unwrap();
        let count = caps["count"].parse::<usize>().unwrap();
        rotate_row(screen, row, count);
    } else if ROTATE_COLUMN_RE.is_match(instruction) {
        let caps = ROTATE_COLUMN_RE.captures(instruction).unwrap();
        let column = caps["column"].parse::<usize>().unwrap();
        let count = caps["count"].parse::<usize>().unwrap();
        rotate_column(screen, column, count);
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let mut screen = vec![vec![Pixel::Off; SCREEN_WIDTH]; SCREEN_HEIGHT];
    input
        .lines()
        .for_each(|line| apply_instruction(&mut screen, line));

    assert_eq!(121, part_1(&screen));
    part_2(&screen);
}
