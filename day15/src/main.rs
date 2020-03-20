struct Disk {
    disk_number: usize,
    total_positions: usize,
    starting_position: usize,
}

impl Disk {
    fn new(disk_number: usize, total_positions: usize, starting_position: usize) -> Self {
        Self {
            disk_number,
            total_positions,
            starting_position,
        }
    }

    fn get_position_after(&self, seconds: usize) -> usize {
        (self.disk_number + self.starting_position + seconds) % self.total_positions
    }
}

fn part_1() -> usize {
    let disks = vec![
        Disk::new(1, 17, 5),
        Disk::new(2, 19, 8),
        Disk::new(3, 7, 1),
        Disk::new(4, 13, 7),
        Disk::new(5, 5, 1),
        Disk::new(6, 3, 0),
    ];

    calculate_time_to_press_button(&disks)
}

fn part_2() -> usize {
    let disks = vec![
        Disk::new(1, 17, 5),
        Disk::new(2, 19, 8),
        Disk::new(3, 7, 1),
        Disk::new(4, 13, 7),
        Disk::new(5, 5, 1),
        Disk::new(6, 3, 0),
        Disk::new(7, 11, 0),
    ];

    calculate_time_to_press_button(&disks)
}

fn calculate_time_to_press_button(disks: &[Disk]) -> usize {
    for second in 0.. {
        let positions = disks
            .iter()
            .map(|disk| disk.get_position_after(second))
            .collect::<Vec<usize>>();
        if positions.windows(2).all(|w| w[0] == w[1]) {
            return second;
        }
    }

    unreachable!()
}

fn main() {
    assert_eq!(16_824, part_1());
    assert_eq!(3_543_984, part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_time_to_press_button() {
        let disks = vec![Disk::new(1, 5, 4), Disk::new(2, 2, 1)];
        assert_eq!(5, calculate_time_to_press_button(&disks));
    }
}
