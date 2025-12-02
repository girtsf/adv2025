fn parse_line(line: &str) -> i32 {
    let (dir, value) = line.split_at(1);
    let value_int = value.parse::<i32>().unwrap();
    match dir {
        "R" => value_int,
        "L" => -value_int,
        _ => panic!("unexpected direction"),
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    dial: i32,
    count_part1: i32,
    count_part2: i32,
}

fn turn(state: State, line: &str) -> State {
    let mut new_state = state;
    let change = parse_line(line);

    let was_zero = state.dial == 0;
    if was_zero && change < 0 {
        // Don't count twice if starting at zero and going L.
        new_state.count_part2 -= 1;
    }

    new_state.dial = state.dial + change;

    while new_state.dial < 0 {
        new_state.dial += 100;
        new_state.count_part2 += 1;
    }
    while new_state.dial >= 100 {
        new_state.dial -= 100;
        if new_state.dial != 0 {
            // Don't count twice if landing on zero going R.
            new_state.count_part2 += 1;
        }
    }
    if new_state.dial == 0 {
        new_state.count_part1 += 1;
        new_state.count_part2 += 1;
    }
    log::info!("{line}: {new_state:?}");
    new_state
}

fn main() {
    let input = adv2025::read_input();
    let initial_state = State {
        dial: 50,
        count_part1: 0,
        count_part2: 0,
    };
    let state = input.lines().fold(initial_state, turn);
    log::info!("{state:?}");
}
