fn parse_horizontal_nums(nums_str: &str, count: usize) -> Vec<Vec<u64>> {
    let mut nums = vec![Vec::<u64>::new(); count];

    nums_str.lines().for_each(|line| {
        line.split_whitespace().enumerate().for_each(|(i, num)| {
            let n = num.parse::<u64>().unwrap();
            nums[i].push(n);
        })
    });
    nums
}

fn parse_vertical_nums(nums_str: &str) -> Vec<Vec<u64>> {
    let lines = nums_str.lines().collect::<Vec<&str>>();
    let num_cols = lines[0].len();
    (0..num_cols)
        .map(|i| {
            lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .filter(|&c| c != ' ')
                .collect::<String>()
                .parse::<u64>()
                .ok()
        })
        .collect::<Vec<Option<u64>>>()
        .split(|x| x.is_none())
        .map(|group| group.iter().map(|&x| x.unwrap()).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>()
}

fn apply_ops(num_nums: &[Vec<u64>], ops: &[&str]) -> u64 {
    num_nums
        .iter()
        .zip(ops.iter())
        .map(|(nums, &op)| match op {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product::<u64>(),
            _ => panic!("Unknown operation"),
        })
        .sum::<u64>()
}

fn main() {
    let input = adv2025::read_input();
    let (nums_str, ops_str) = input.trim_end().rsplit_once('\n').unwrap();
    let ops = ops_str.split_whitespace().collect::<Vec<&str>>();
    let part1_nums = parse_horizontal_nums(nums_str, ops.len());
    let part2_nums = parse_vertical_nums(nums_str);

    dbg!(apply_ops(&part1_nums, &ops));
    dbg!(apply_ops(&part2_nums, &ops));
}
