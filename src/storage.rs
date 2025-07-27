use std::fs::File;
use std::io::{self, BufRead, Write};
use chrono::NaiveDate;
use crate::task::Task;

pub fn load_tasks_from_file(path: &str) -> io::Result<Vec<Task>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut tasks = Vec::new();

    for line in reader.lines().skip(3) {
        let line = line?;
        if line.starts_with('+') || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim_matches('|').split('|').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            if let Ok(date) = NaiveDate::parse_from_str(parts[2], "%Y-%m-%d") {
                tasks.push(Task {
                    description: parts[0].to_string(),
                    status: parts[1].to_string(),
                    deadline: date,
                    reminder: parts[3].to_string(),
                });
            }
        }
    }

    Ok(tasks)
}

pub fn save_to_text_file(path: &str, tasks: &[Task]) -> io::Result<()> {
    let mut file = File::create(path)?;
    let headers = ["Todo", "Status", "Deadline", "Reminder Notification"];

    writeln!(file, "+{:-<25}+{:-<15}+{:-<12}+{:-<24}+", "", "", "", "")?;
    writeln!(
        file,
        "| {:<25} | {:<15} | {:<12} | {:<24} |",
        headers[0], headers[1], headers[2], headers[3]
    )?;
    writeln!(file, "+{:-<25}+{:-<15}+{:-<12}+{:-<24}+", "", "", "", "")?;

    for task in tasks {
        writeln!(
            file,
            "| {:<25} | {:<15} | {:<12} | {:<24} |",
            task.description,
            task.status,
            task.deadline.format("%Y-%m-%d"),
            task.reminder
        )?;
    }

    writeln!(file, "+{:-<25}+{:-<15}+{:-<12}+{:-<24}+", "", "", "", "")?;
    Ok(())
}
