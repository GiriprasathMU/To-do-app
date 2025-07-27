use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub status: String,
    pub deadline: NaiveDate,
    pub reminder: String,
}
