use crate::task::Task;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub enum Operation {
    DeleteTask {
        task: Task,
        original_date: NaiveDate,
    },
    EditTask {
        task_id: String,
        old_task: Task,
        new_task: Task,
    },
    CreateTask {
        task: Task,
    },
    YankPaste {
        task_id: String,
        old_date: NaiveDate,
        new_date: NaiveDate,    
    },
    // Add more operations as needed
}

#[derive(Debug, Clone)]
pub struct UndoStack {
    operations: Vec<Operation>,
    max_size: usize,
}

impl UndoStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            operations: Vec::new(),
            max_size,
        }
    }
    
    pub fn push(&mut self, operation: Operation) {
        self.operations.push(operation);
        
        // Keep stack size under control
        if self.operations.len() > self.max_size {
            self.operations.remove(0);
        }
    }
    
    pub fn pop(&mut self) -> Option<Operation> {
        self.operations.pop()
    }
    
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.operations.clear();
    }
    
    pub fn len(&self) -> usize {
        self.operations.len()
    }
}

impl Operation {
    pub fn get_description(&self) -> String {
        match self {
            Operation::DeleteTask { task, .. } => format!("Delete '{}'", task.title),
            Operation::EditTask { old_task, .. } => format!("Edit '{}'", old_task.title),
            Operation::CreateTask { task } => format!("Create '{}'", task.title),
            Operation::YankPaste { task_id, .. } => format!("Move task '{}'", task_id),
        }
    }
}
