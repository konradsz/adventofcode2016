use std::collections::VecDeque;

fn part_1(elves_count: usize) -> usize {
    let mut v = Vec::with_capacity(elves_count);
    for i in 1..=elves_count {
        v.push((i, true));
    }

    while v.len() != 1 {
        for (_, available) in v
            .iter_mut()
            .filter(|(_, available)| *available)
            .skip(1)
            .step_by(2)
        {
            *available = false;
        }

        if v.len() % 2 != 0 {
            *v.iter_mut().next().unwrap() = (0, false);
        }

        v.retain(|&(_, available)| available);
    }
    v[0].0
}

fn part_2(elves_count: usize) -> usize {
    let mut v = VecDeque::with_capacity(elves_count);
    for i in 1..=elves_count {
        v.push_back((i, true));
    }

    while v.len() != 1 {
        for removed in 0..(v.len()) / 2 {
            let index = (v.len() + removed) / 2;
            v[index].1 = false;
            if v.len() != 1 {
                let front = v.pop_front().unwrap();
                v.push_back(front);
            }
        }

        v.retain(|&(_, available)| available);
    }

    v[0].0
}

fn main() {
    const ELVES: usize = 3_004_953;
    assert_eq!(1_815_603, part_1(ELVES));
    assert_eq!(1_410_630, part_2(ELVES));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        const ELVES: usize = 5;
        assert_eq!(3, part_1(ELVES));
    }

    #[test]
    fn test_part_2() {
        const ELVES: usize = 5;
        assert_eq!(2, part_2(ELVES));
    }
}
