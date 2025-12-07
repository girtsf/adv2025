use std::ops::RangeInclusive;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            RangeInclusive::new(start, end)
        })
        .collect()
}

fn part2(ranges: &mut Vec<RangeInclusive<u64>>) -> usize {
    let mut count = 0usize;
    ranges.sort_by_key(|r| *r.start());
    let mut current_start = *ranges[0].start();
    let mut current_end = *ranges[0].end();
    for range in ranges.iter().skip(1) {
        let start = *range.start();
        let end = *range.end();
        if start > current_end + 1 {
            count += (current_end - current_start + 1) as usize;
            current_start = start;
            current_end = end;
        } else if end > current_end {
            current_end = end;
        }
    }
    count += (current_end - current_start + 1) as usize;
    count
}

fn main() {
    let input = adv2025::read_input();
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(ranges_str);
    let ids: Vec<u64> = ids_str.lines().map(|line| line.parse().unwrap()).collect();
    let part1: usize = ids
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();
    dbg!(part1);
    dbg!(part2(&mut ranges));
}
