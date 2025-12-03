use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let (a_s, b_s) = s.split_once('-').unwrap();
            let a = a_s.parse::<u64>().unwrap();
            let b = b_s.parse::<u64>().unwrap();
            a..=b
        })
        .collect()
}

fn is_repeated_sequence(n: u64, seq_size: usize) -> bool {
    let s = n.to_string();
    let first = &s[0..seq_size];
    let mut idx = seq_size;
    while idx + seq_size <= s.len() {
        let next = &s[idx..idx + seq_size];
        if next != first {
            // log::info!("{n} is not repeated sequence of size {seq_size}");
            return false;
        }
        idx += seq_size;
    }
    // log::info!("{n} is repeated sequence of size {seq_size}");
    true
}

fn is_id(n: u64, part2: bool) -> bool {
    let s = n.to_string();
    for seq_len in 1..=s.len() / 2 {
        if !part2 && (s.len() / seq_len) != 2 {
            continue;
        }
        if s.len() % seq_len != 0 {
            continue;
        }
        if is_repeated_sequence(n, seq_len) {
            return true;
        }
    }
    false
}

fn sum_ids_in_range(range: &RangeInclusive<u64>, part2: bool) -> u64 {
    range
        .clone()
        .filter_map(|n| is_id(n, part2).then_some(n))
        .sum()
}

fn sum_ids_in_ranges(ranges: &[RangeInclusive<u64>], part2: bool) -> u64 {
    ranges.iter().map(|r| sum_ids_in_range(r, part2)).sum()
}

fn main() {
    let input = adv2025::read_input();
    let parsed = parse_input(&input);
    dbg!(sum_ids_in_ranges(&parsed, false));
    dbg!(sum_ids_in_ranges(&parsed, true));
}
