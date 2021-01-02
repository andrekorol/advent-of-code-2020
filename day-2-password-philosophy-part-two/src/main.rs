use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let puzzle_input = fs::read_to_string("puzzle-input.txt")?;
    let passwords: Vec<&str> = puzzle_input
        .lines()
        .filter_map(|s| Some(s.trim()))
        .collect();

    let mut valid_passords = 0;
    for passwd_policy in passwords {
        let policy_parts = passwd_policy.split_whitespace().collect::<Vec<_>>();

        let pos1: usize;
        let pos2: usize;
        match policy_parts[0].split("-").collect::<Vec<_>>().as_slice() {
            [first, second] => {
                pos1 = first.to_string().parse::<usize>().unwrap() - 1;
                pos2 = second.to_string().parse::<usize>().unwrap() - 1;
            }
            _ => unreachable!(),
        }

        let letter = policy_parts[1].split(":").collect::<Vec<_>>()[0];

        let psswd = policy_parts[2];

        let mut occurrences = 0;

        for (i, c) in psswd.char_indices() {
            if i == pos1 && c.to_string() == letter {
                occurrences += 1;
            }
            if i == pos2 && c.to_string() == letter {
                occurrences += 1;
            }
        }
        if occurrences == 1 {
            valid_passords += 1;
        }
    }

    println!("Result = {} valid passwords", valid_passords);

    Ok(())
}
