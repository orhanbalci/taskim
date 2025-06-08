use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub comments: Vec<TaskComment>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskComment {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub events: Vec<Task>,
}

impl Task {
    pub fn new(title: String, start: DateTime<Utc>) -> Self {
        let id = Uuid::new_v4().to_string();
        let end = start + chrono::Duration::hours(1);
        
        Self {
            id,
            title,
            start,
            end,
            comments: vec![],
            completed: false,
        }
    }
    
    pub fn add_comment(&mut self, text: String) {
        let comment = TaskComment {
            id: Uuid::new_v4().to_string(),
            text,
        };
        self.comments.push(comment);
    }
    
    pub fn is_on_date(&self, date: chrono::NaiveDate) -> bool {
        let task_date = self.start.date_naive();
        task_date == date
    }
}

impl Default for TaskData {
    fn default() -> Self {
        Self {
            events: vec![],
        }
    }
}
