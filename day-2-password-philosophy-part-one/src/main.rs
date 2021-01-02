use std::{convert::TryInto, fs};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let puzzle_input = fs::read_to_string("puzzle-input.txt")?;
    let passwords: Vec<&str> = puzzle_input
        .lines()
        .filter_map(|s| Some(s.trim()))
        .collect();

    let mut valid_passords = 0;
    for passwd_policy in passwords {
        let policy_parts = passwd_policy.split_whitespace().collect::<Vec<_>>();

        let min: u32;
        let max: u32;
        match policy_parts[0].split("-").collect::<Vec<_>>().as_slice() {
            [first, second] => {
                min = first.to_string().parse().unwrap();
                max = second.to_string().parse().unwrap();
            }
            _ => unreachable!(),
        }

        let letter = policy_parts[1].split(":").collect::<Vec<_>>()[0];

        let psswd = policy_parts[2];

        let letter_count: u32 = psswd.matches(letter).count().try_into().unwrap();
        if letter_count >= min && letter_count <= max {
            valid_passords += 1;
        }
    }

    println!("Result = {} valid passwords", valid_passords);

    Ok(())
}
