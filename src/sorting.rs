use crate::task::Task;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Description,
    Status,
    Deadline,
    Reminder,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}


pub fn sort_tasks(tasks: &mut Vec<Task>, field: &SortField, order: &SortOrder) {
    match field {
        SortField::Description => tasks.sort_by_key(|t| t.description.clone()),
        SortField::Status => tasks.sort_by_key(|t| t.status.clone()),
        SortField::Deadline => tasks.sort_by_key(|t| t.deadline),
        SortField::Reminder => tasks.sort_by_key(|t| t.reminder.clone()),
    }

    if let SortOrder::Descending = order {
        tasks.reverse();
    }
}
