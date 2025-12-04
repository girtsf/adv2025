type Map = Vec<Vec<char>>;

use adv2025::Pos;

fn parse_input(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_removable_rolls(map: &Map) -> Vec<Pos> {
    let mut rolls = Vec::new();
    let size = adv2025::Pos::new(map.len(), map[0].len());

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x];
            if c != '@' {
                continue;
            }
            let pos = adv2025::Pos::new(y, x);
            let adjacent_rolls = pos
                .all_neighbors()
                .iter()
                .filter(|pos| pos.check_bounds(&size))
                .map(|pos| {
                    if map[pos.y as usize][pos.x as usize] == '@' {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>();
            if adjacent_rolls < 4 {
                rolls.push(pos);
            }
        }
    }
    rolls
}

fn part1(map: &Map) -> usize {
    find_removable_rolls(map).len()
}

fn part2(map: &Map) -> usize {
    let mut count = 0;

    let mut current_map = map.clone();
    loop {
        let removable_rolls = find_removable_rolls(&current_map);
        if removable_rolls.is_empty() {
            return count;
        }
        count += removable_rolls.len();
        for pos in removable_rolls {
            current_map[pos.y as usize][pos.x as usize] = '.';
        }
    }
}

fn main() {
    let input = adv2025::read_input();
    let map = parse_input(&input);
    dbg!(part1(&map));
    dbg!(part2(&map));
}
