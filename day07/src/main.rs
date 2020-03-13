use std::fs;

struct AddressIPv7<'a> {
    supernet_sequences: Vec<&'a str>,
    hypernet_sequences: Vec<&'a str>,
}

impl<'a> AddressIPv7<'a> {
    fn new(address: &str) -> AddressIPv7 {
        let opening: Vec<usize> = address.match_indices('[').map(|m| m.0).collect();
        let closing: Vec<usize> = address.match_indices(']').map(|m| m.0).collect();
        let braces: Vec<(usize, usize)> = opening.into_iter().zip(closing.into_iter()).collect();

        let hypernet_sequences = braces
            .iter()
            .map(|&(start, end)| &address[start + 1..end])
            .collect::<Vec<&str>>();

        let mut non_braces = Vec::new();
        let mut start = 0;
        for (open, close) in braces.iter() {
            non_braces.push((start, *open));
            start = close + 1;
        }
        non_braces.push((start, address.len()));

        let supernet_sequences = non_braces
            .iter()
            .map(|&(start, end)| &address[start..end])
            .collect::<Vec<&str>>();

        AddressIPv7 {
            supernet_sequences,
            hypernet_sequences,
        }
    }
}

fn part_1(addresses: &[AddressIPv7]) -> usize {
    addresses.iter().filter(supports_tls).count()
}

fn part_2(addresses: &[AddressIPv7]) -> usize {
    addresses.iter().filter(supports_ssl).count()
}

fn supports_tls(address: &&AddressIPv7) -> bool {
    let has_abba = |sequence: &&str| {
        sequence.as_bytes().windows(4).any(|window| {
            window[0] != window[1] && window[0] == window[3] && window[1] == window[2]
        })
    };

    address.supernet_sequences.iter().any(has_abba)
        && !address.hypernet_sequences.iter().any(has_abba)
}

fn supports_ssl(address: &&AddressIPv7) -> bool {
    let is_aba = |window: &[u8]| window[0] == window[2] && window[0] != window[1];
    let is_bab = |super_window: &[u8], hyper_window: &[u8]| {
        hyper_window[0] == hyper_window[2]
            && hyper_window[0] != hyper_window[1]
            && hyper_window[0] == super_window[1]
            && hyper_window[1] == super_window[0]
    };

    address.supernet_sequences.iter().any(|super_seq| {
        super_seq.as_bytes().windows(3).any(|super_window| {
            if is_aba(super_window) {
                return address.hypernet_sequences.iter().any(|hyper_seq| {
                    hyper_seq
                        .as_bytes()
                        .windows(3)
                        .any(|hyper_window| is_bab(super_window, hyper_window))
                });
            }
            false
        })
    })
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let addresses: Vec<AddressIPv7> = input.lines().map(AddressIPv7::new).collect();

    assert_eq!(105, part_1(&addresses));
    assert_eq!(258, part_2(&addresses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_tls() {
        assert!(supports_tls(&&AddressIPv7::new("abba[mnop]qrst")));
        assert!(!supports_tls(&&AddressIPv7::new("abcd[bddb]xyyx")));
        assert!(!supports_tls(&&AddressIPv7::new("aaaa[qwer]tyui")));
        assert!(supports_tls(&&AddressIPv7::new("ioxxoj[asdfgh]zxcvbn")));
    }

    #[test]
    fn test_supports_ssl() {
        assert!(supports_ssl(&&AddressIPv7::new("aba[bab]xyz")));
        assert!(!supports_ssl(&&AddressIPv7::new("xyx[xyx]xyx")));
        assert!(supports_ssl(&&AddressIPv7::new("aaa[kek]eke")));
        assert!(supports_ssl(&&AddressIPv7::new("zazbz[bzb]cdb")));
    }
}
