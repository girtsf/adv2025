fn find_largest_joltage(joltages: &[u8], count: usize) -> u64 {
    if count == 0 {
        return 0;
    }
    let sub = &joltages[0..(joltages.len() - count + 1)];
    let (idx, value) = sub
        .iter()
        .enumerate()
        .max_by_key(|&(idx, v)| (v, -(idx as isize)))
        .unwrap();
    10u64.pow(count as u32 - 1) * (*value as u64)
        + find_largest_joltage(&joltages[idx + 1..], count - 1)
}

fn parse_line(s: &str) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn find_total_joltage(joltages: &[Vec<u8>], count: usize) -> u64 {
    joltages
        .iter()
        .map(|j| find_largest_joltage(j, count))
        .sum()
}

fn main() {
    let input = adv2025::read_input();
    let parsed = input.lines().map(|l| parse_line(l)).collect::<Vec<_>>();
    dbg!(find_total_joltage(&parsed, 2));
    dbg!(find_total_joltage(&parsed, 12));
}
