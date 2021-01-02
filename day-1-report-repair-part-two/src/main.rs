use std::{collections::HashSet, fs};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let expense_report = fs::read_to_string("expense-report.txt")?;
    let entries: Vec<u32> = expense_report
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Solution: Combinational sum using a hash table
    const SUM: u32 = 2020;
    // let mut x;
    for i in 0..entries.len() - 2 {
        let mut s = HashSet::new();
        let current_sum = SUM - entries[i];
        for j in i + 1..entries.len() {
            if s.contains(&(current_sum - entries[j])) {
                println!(
                    "Result = {}",
                    entries[i] * entries[j] * (current_sum - entries[j])
                );
                return Ok(());
            }
            s.insert(entries[j]);
        }
    }
    Ok(())
}
