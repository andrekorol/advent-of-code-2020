use std::{collections::HashSet, fs};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let expense_report = fs::read_to_string("expense-report.txt")?;
    let entries: Vec<u32> = expense_report
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Solution: Combinational sum using a hash table
    let mut looked_entries = HashSet::new();
    const K: u32 = 2020;
    let mut x;
    for i in entries {
        x = K - i;
        if looked_entries.contains(&x) {
            println!("Result = {}", x * i);
            return Ok(());
        }
        looked_entries.insert(i);
    }
    Ok(())
}
