use crate::core::read;

pub fn report_full() {
    read()
        .into_iter()
        .for_each(|entry| println!("{} {} {}", entry.hours, entry.task, entry.budget));
}