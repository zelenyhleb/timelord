use crate::core::{log_info, read, write, Entry};

pub fn record(hours: u8, budget: &str, task: &str) {
    write_single(Entry {
        hours: hours,
        budget: budget.to_owned(),
        task: task.to_owned(),
    });
}

fn write_single(entry: Entry) {
    let mut existing = read();
    log_info(
        format!(
            "recording {} hours on task {} to budget {}",
            &entry.hours, &entry.task, &entry.budget
        )
        .as_str(),
    );
    existing.push(entry);
    write(existing);
    log_info("success");
}
