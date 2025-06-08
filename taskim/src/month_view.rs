use crate::task::Task;
use chrono::{Datelike, NaiveDate};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionType {
    Day(NaiveDate),
    Task(String), // task id
    #[allow(dead_code)]
    WeekGoal(String), // week key
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub selection_type: SelectionType,
    #[allow(dead_code)]
    pub task_index_in_day: Option<usize>,
}

pub struct MonthView {
    pub current_date: NaiveDate,
    pub selection: Selection,
    pub weeks: Vec<Vec<NaiveDate>>,
}

impl MonthView {
    pub fn new(current_date: NaiveDate) -> Self {
        let weeks = Self::build_weeks(current_date);
        let selection = Selection {
            selection_type: SelectionType::Day(current_date),
            task_index_in_day: None,
        };
        
        Self {
            current_date,
            selection,
            weeks,
        }
    }
    
    // Public method to rebuild weeks for a given date
    pub fn build_weeks_for_date(date: NaiveDate) -> Vec<Vec<NaiveDate>> {
        Self::build_weeks(date)
    }
    
    fn build_weeks(date: NaiveDate) -> Vec<Vec<NaiveDate>> {
        let first_of_month = date.with_day(1).unwrap();
        let last_of_month = date.with_day(
            match date.month() {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => if date.year() % 4 == 0 && (date.year() % 100 != 0 || date.year() % 400 == 0) { 29 } else { 28 },
                _ => unreachable!(),
            }
        ).unwrap();
        
        // Start from the first Sunday of the month view
        let mut start_date = first_of_month;
        while start_date.weekday().num_days_from_sunday() != 0 {
            start_date = start_date.pred_opt().unwrap();
        }
        
        let mut weeks = Vec::new();
        let mut current_date = start_date;
        
        // Build 6 weeks to ensure we cover the entire month
        for _ in 0..6 {
            let mut week = Vec::new();
            for _ in 0..7 {
                week.push(current_date);
                current_date = current_date.succ_opt().unwrap();
            }
            weeks.push(week);
            
            // If we've passed the end of the month and filled at least 4 weeks, we can stop
            if current_date > last_of_month && weeks.len() >= 4 {
                break;
            }
        }
        
        weeks
    }
    
    pub fn next_month(&mut self) {
        self.current_date = if self.current_date.month() == 12 {
            NaiveDate::from_ymd_opt(self.current_date.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month() + 1, 1).unwrap()
        };
        self.weeks = Self::build_weeks(self.current_date);
        self.selection = Selection {
            selection_type: SelectionType::Day(self.current_date),
            task_index_in_day: None,
        };
    }
    
    pub fn prev_month(&mut self) {
        self.current_date = if self.current_date.month() == 1 {
            NaiveDate::from_ymd_opt(self.current_date.year() - 1, 12, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month() - 1, 1).unwrap()
        };
        self.weeks = Self::build_weeks(self.current_date);
        self.selection = Selection {
            selection_type: SelectionType::Day(self.current_date),
            task_index_in_day: None,
        };
    }
    
    pub fn next_year(&mut self) {
        self.current_date = NaiveDate::from_ymd_opt(self.current_date.year() + 1, self.current_date.month(), 1).unwrap();
        self.weeks = Self::build_weeks(self.current_date);
        self.selection = Selection {
            selection_type: SelectionType::Day(self.current_date),
            task_index_in_day: None,
        };
    }
    
    pub fn prev_year(&mut self) {
        self.current_date = NaiveDate::from_ymd_opt(self.current_date.year() - 1, self.current_date.month(), 1).unwrap();
        self.weeks = Self::build_weeks(self.current_date);
        self.selection = Selection {
            selection_type: SelectionType::Day(self.current_date),
            task_index_in_day: None,
        };
    }
    
    fn get_week_key(date: NaiveDate) -> String {
        let year = date.year();
        let week = date.iso_week().week();
        format!("{:04}-{:02}", year, week)
    }
    
    pub fn move_up(&mut self, tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                let current_date = *date;
                // Check if moving up a week would go outside current month
                if let Some(new_date) = current_date.checked_sub_signed(chrono::Duration::weeks(1)) {
                    if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                        // Go to previous month
                        let target_day = current_date.day();
                        self.prev_month();
                        // Try to find a similar date in the new month
                        let days_in_month = match self.current_date.month() {
                            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                            4 | 6 | 9 | 11 => 30,
                            2 => if self.current_date.year() % 4 == 0 && (self.current_date.year() % 100 != 0 || self.current_date.year() % 400 == 0) { 29 } else { 28 },
                            _ => 31,
                        };
                        let safe_day = std::cmp::min(target_day, days_in_month);
                        if let Some(target_date) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), safe_day) {
                            self.selection = Selection {
                                selection_type: SelectionType::Day(target_date),
                                task_index_in_day: None,
                            };
                        }
                    } else {
                        self.selection = Selection {
                            selection_type: SelectionType::Day(new_date),
                            task_index_in_day: None,
                        };
                    }
                }
            }
            SelectionType::Task(task_id) => {
                let task_id = task_id.clone();
                // Find the current task and move to previous task in the same day
                if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                    let task_date = task.start.date_naive();
                    let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(task_date)).collect();
                    
                    if let Some(current_index) = day_tasks.iter().position(|t| t.id == task_id) {
                        if current_index > 0 {
                            // Move to previous task
                            let prev_task = &day_tasks[current_index - 1];
                            self.selection = Selection {
                                selection_type: SelectionType::Task(prev_task.id.clone()),
                                task_index_in_day: Some(current_index - 1),
                            };
                        } else {
                            // Move to day selection
                            self.selection = Selection {
                                selection_type: SelectionType::Day(task_date),
                                task_index_in_day: None,
                            };
                        }
                    }
                }
            }
            SelectionType::WeekGoal(_) => {
                // Move from week goal to the day above it
                // This would be the last day of the previous week
            }
        }
    }
    
    pub fn move_down(&mut self, tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                let current_date = *date;
                // Check if there are tasks on this day
                let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(current_date)).collect();
                if !day_tasks.is_empty() {
                    // Move to first task
                    self.selection = Selection {
                        selection_type: SelectionType::Task(day_tasks[0].id.clone()),
                        task_index_in_day: Some(0),
                    };
                } else {
                    // Check if moving down a week would go outside current month
                    if let Some(new_date) = current_date.checked_add_signed(chrono::Duration::weeks(1)) {
                        if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                            // Go to next month
                            let target_day = current_date.day();
                            self.next_month();
                            // Try to find a similar date in the new month
                            let days_in_month = match self.current_date.month() {
                                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                                4 | 6 | 9 | 11 => 30,
                                2 => if self.current_date.year() % 4 == 0 && (self.current_date.year() % 100 != 0 || self.current_date.year() % 400 == 0) { 29 } else { 28 },
                                _ => 31,
                            };
                            let safe_day = std::cmp::min(target_day, days_in_month);
                            if let Some(target_date) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), safe_day) {
                                self.selection = Selection {
                                    selection_type: SelectionType::Day(target_date),
                                    task_index_in_day: None,
                                };
                            }
                        } else {
                            self.selection = Selection {
                                selection_type: SelectionType::Day(new_date),
                                task_index_in_day: None,
                            };
                        }
                    }
                }
            }
            SelectionType::Task(task_id) => {
                let task_id = task_id.clone();
                // Find the current task and move to next task in the same day or to next week
                if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                    let task_date = task.start.date_naive();
                    let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(task_date)).collect();
                    
                    if let Some(current_index) = day_tasks.iter().position(|t| t.id == task_id) {
                        if current_index < day_tasks.len() - 1 {
                            // Move to next task
                            let next_task = &day_tasks[current_index + 1];
                            self.selection = Selection {
                                selection_type: SelectionType::Task(next_task.id.clone()),
                                task_index_in_day: Some(current_index + 1),
                            };
                        } else {
                            // Move to next week same day
                            if let Some(new_date) = task_date.checked_add_signed(chrono::Duration::weeks(1)) {
                                if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                                    // Go to next month
                                    let target_day = task_date.day();
                                    self.next_month();
                                    let days_in_month = match self.current_date.month() {
                                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                                        4 | 6 | 9 | 11 => 30,
                                        2 => if self.current_date.year() % 4 == 0 && (self.current_date.year() % 100 != 0 || self.current_date.year() % 400 == 0) { 29 } else { 28 },
                                        _ => 31,
                                    };
                                    let safe_day = std::cmp::min(target_day, days_in_month);
                                    if let Some(target_date) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), safe_day) {
                                        self.selection = Selection {
                                            selection_type: SelectionType::Day(target_date),
                                            task_index_in_day: None,
                                        };
                                    }
                                } else {
                                    self.selection = Selection {
                                        selection_type: SelectionType::Day(new_date),
                                        task_index_in_day: None,
                                    };
                                }
                            }
                        }
                    }
                }
            }
            SelectionType::WeekGoal(_) => {
                // Move to next week goal
            }
        }
    }
    
    pub fn move_left(&mut self, _tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                if let Some(new_date) = date.checked_sub_signed(chrono::Duration::days(1)) {
                    // Check if we're moving to a different month
                    if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                        // Go to previous month and select the last day
                        self.prev_month();
                        let days_in_month = match self.current_date.month() {
                            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                            4 | 6 | 9 | 11 => 30,
                            2 => if self.current_date.year() % 4 == 0 && (self.current_date.year() % 100 != 0 || self.current_date.year() % 400 == 0) { 29 } else { 28 },
                            _ => 31,
                        };
                        if let Some(last_day) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), days_in_month) {
                            self.selection = Selection {
                                selection_type: SelectionType::Day(last_day),
                                task_index_in_day: None,
                            };
                        }
                    } else {
                        self.selection = Selection {
                            selection_type: SelectionType::Day(new_date),
                            task_index_in_day: None,
                        };
                    }
                }
            }
            SelectionType::Task(task_id) => {
                // Move to the day containing this task
                if let Some(task) = _tasks.iter().find(|t| &t.id == task_id) {
                    let task_date = task.start.date_naive();
                    self.selection = Selection {
                        selection_type: SelectionType::Day(task_date),
                        task_index_in_day: None,
                    };
                }
            }
            SelectionType::WeekGoal(_) => {
                // Stay on week goal
            }
        }
    }
    
    pub fn move_right(&mut self, _tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                if let Some(new_date) = date.checked_add_signed(chrono::Duration::days(1)) {
                    // Check if we're moving to a different month
                    if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                        // Go to next month and select the first day
                        self.next_month();
                        if let Some(first_day) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), 1) {
                            self.selection = Selection {
                                selection_type: SelectionType::Day(first_day),
                                task_index_in_day: None,
                            };
                        }
                    } else {
                        self.selection = Selection {
                            selection_type: SelectionType::Day(new_date),
                            task_index_in_day: None,
                        };
                    }
                }
            }
            SelectionType::Task(task_id) => {
                // Move to the day containing this task
                if let Some(task) = _tasks.iter().find(|t| &t.id == task_id) {
                    let task_date = task.start.date_naive();
                    self.selection = Selection {
                        selection_type: SelectionType::Day(task_date),
                        task_index_in_day: None,
                    };
                }
            }
            SelectionType::WeekGoal(_) => {
                // Stay on week goal
            }
        }
    }
    
    pub fn get_selected_task_id(&self) -> Option<String> {
        match &self.selection.selection_type {
            SelectionType::Task(task_id) => Some(task_id.clone()),
            _ => None,
        }
    }

    // Get the currently selected date
    pub fn get_selected_date(&self) -> NaiveDate {
        match &self.selection.selection_type {
            SelectionType::Day(date) => *date,
            SelectionType::Task(_task_id) => {
                // If a task is selected, return the current date as fallback
                // In a real implementation, you might want to find the task's date
                self.current_date
            }
            SelectionType::WeekGoal(_) => self.current_date,
        }
    }

    // Move to next week (same day of week)
    pub fn next_week(&mut self) {
        let current_selected = self.get_selected_date();
        if let Some(new_date) = current_selected.checked_add_signed(chrono::Duration::weeks(1)) {
            // Check if we need to change months
            if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                self.current_date = new_date.with_day(1).unwrap();
                self.weeks = Self::build_weeks(self.current_date);
            }
            self.selection = Selection {
                selection_type: SelectionType::Day(new_date),
                task_index_in_day: None,
            };
        }
    }

    // Move to previous week (same day of week)
    pub fn prev_week(&mut self) {
        let current_selected = self.get_selected_date();
        if let Some(new_date) = current_selected.checked_sub_signed(chrono::Duration::weeks(1)) {
            // Check if we need to change months
            if new_date.month() != self.current_date.month() || new_date.year() != self.current_date.year() {
                self.current_date = new_date.with_day(1).unwrap();
                self.weeks = Self::build_weeks(self.current_date);
            }
            self.selection = Selection {
                selection_type: SelectionType::Day(new_date),
                task_index_in_day: None,
            };
        }
    }

    // Move to first day of current month
    pub fn first_day_of_month(&mut self) {
        let first_day = self.current_date.with_day(1).unwrap();
        self.selection = Selection {
            selection_type: SelectionType::Day(first_day),
            task_index_in_day: None,
        };
    }

    // Move to last day of current month
    pub fn last_day_of_month(&mut self) {
        let days_in_month = match self.current_date.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.current_date.year() % 4 == 0 && (self.current_date.year() % 100 != 0 || self.current_date.year() % 400 == 0) { 29 } else { 28 },
            _ => 31,
        };
        
        if let Some(last_day) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), days_in_month) {
            self.selection = Selection {
                selection_type: SelectionType::Day(last_day),
                task_index_in_day: None,
            };
        }
    }

    // Navigate to previous month while preserving the current day when possible
    pub fn prev_month_preserve_day(&mut self) {
        let current_selected = self.get_selected_date();
        let target_day = current_selected.day();
        
        let (new_year, new_month) = if self.current_date.month() == 1 {
            (self.current_date.year() - 1, 12)
        } else {
            (self.current_date.year(), self.current_date.month() - 1)
        };
        
        // Calculate days in the target month
        let days_in_month = match new_month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if new_year % 4 == 0 && (new_year % 100 != 0 || new_year % 400 == 0) { 29 } else { 28 },
            _ => 31,
        };
        
        // Preserve day or use last day of month if target day doesn't exist
        let safe_day = std::cmp::min(target_day, days_in_month);
        
        self.current_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).unwrap();
        self.weeks = Self::build_weeks(self.current_date);
        
        if let Some(target_date) = NaiveDate::from_ymd_opt(new_year, new_month, safe_day) {
            self.selection = Selection {
                selection_type: SelectionType::Day(target_date),
                task_index_in_day: None,
            };
        }
    }

    // Navigate to next month while preserving the current day when possible
    pub fn next_month_preserve_day(&mut self) {
        let current_selected = self.get_selected_date();
        let target_day = current_selected.day();
        
        let (new_year, new_month) = if self.current_date.month() == 12 {
            (self.current_date.year() + 1, 1)
        } else {
            (self.current_date.year(), self.current_date.month() + 1)
        };
        
        // Calculate days in the target month
        let days_in_month = match new_month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if new_year % 4 == 0 && (new_year % 100 != 0 || new_year % 400 == 0) { 29 } else { 28 },
            _ => 31,
        };
        
        // Preserve day or use last day of month if target day doesn't exist
        let safe_day = std::cmp::min(target_day, days_in_month);
        
        self.current_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).unwrap();
        self.weeks = Self::build_weeks(self.current_date);
        
        if let Some(target_date) = NaiveDate::from_ymd_opt(new_year, new_month, safe_day) {
            self.selection = Selection {
                selection_type: SelectionType::Day(target_date),
                task_index_in_day: None,
            };
        }
    }

    // Navigate to today's date
    pub fn go_to_today(&mut self) {
        use chrono::Local;
        
        let today = Local::now().date_naive();
        
        // Update current_date and rebuild weeks if we're in a different month/year
        if today.month() != self.current_date.month() || today.year() != self.current_date.year() {
            self.current_date = today.with_day(1).unwrap();
            self.weeks = Self::build_weeks(self.current_date);
        }
        
        self.selection = Selection {
            selection_type: SelectionType::Day(today),
            task_index_in_day: None,
        };
    }
}

pub fn render_month_view(
    frame: &mut Frame,
    area: Rect,
    month_view: &MonthView,
    tasks: &[Task],
    weekly_goals: &HashMap<String, String>,
) {
    let title = format!(
        "{} {}",
        month_view.current_date.format("%B"),
        month_view.current_date.year()
    );
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    
    let inner_area = block.inner(area);
    frame.render_widget(block, area);
    
    // Calculate constraints for each week individually based on max tasks in that week
    let week_constraints: Vec<Constraint> = month_view.weeks.iter()
        .map(|week| {
            // Calculate max tasks for this specific week
            let max_tasks_in_week = week.iter()
                .map(|&date| {
                    let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(date)).collect();
                    day_tasks.len()
                })
                .max()
                .unwrap_or(0);
            
            // Each week needs:
            // - 1 row for weekly goal
            // - Minimum 4 rows for day cells (1 for day number + 3 minimum for tasks/spacing)
            // - Additional rows for tasks (unlimited expansion based on actual task count in this week)
            let minimum_week_height = 5; // goal (1) + minimum day cell height (4 lines minimum)
            let base_week_height = 2; // goal + day number base
            let additional_task_rows = max_tasks_in_week; // Unlimited expansion based on actual tasks
            let calculated_height = base_week_height + additional_task_rows.max(2); // Ensure at least 2 rows for tasks
            let week_height = calculated_height.max(minimum_week_height); // Enforce absolute minimum
            
            Constraint::Length(week_height as u16)
        })
        .collect();
    
    let week_layout = Layout::vertical(week_constraints).split(inner_area);
    
    for (week_index, week) in month_view.weeks.iter().enumerate() {
        if week_index >= week_layout.len() {
            break;
        }
        
        let week_area = week_layout[week_index];
        
        // Calculate the height for this specific week (recalculate to match constraint logic)
        let max_tasks_in_week = week.iter()
            .map(|&date| {
                let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(date)).collect();
                day_tasks.len()
            })
            .max()
            .unwrap_or(0);
        let minimum_week_height = 5;
        let base_week_height = 2;
        let additional_task_rows = max_tasks_in_week;
        let calculated_height = base_week_height + additional_task_rows.max(2);
        let calculated_week_height = calculated_height.max(minimum_week_height);
        
        // Split week area into goal row and day row
        let week_split = Layout::vertical([
            Constraint::Length(1), 
            Constraint::Length((calculated_week_height.saturating_sub(1)).max(4) as u16) // Ensure minimum 4 lines for day cells
        ]).split(week_area);
        
        let goal_area = week_split[0];
        let day_area = week_split[1];
        
        // Render weekly goal
        let week_key = MonthView::get_week_key(week[0]);
        let goal_text = weekly_goals.get(&week_key).cloned().unwrap_or_default();
        let goal_style = if matches!(month_view.selection.selection_type, SelectionType::WeekGoal(ref key) if key == &week_key) {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let goal_paragraph = Paragraph::new(goal_text)
            .style(goal_style)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(goal_paragraph, goal_area);
        
        // Render days
        let day_constraints: Vec<Constraint> = (0..7)
            .map(|_| Constraint::Percentage(100 / 7))
            .collect();
        
        let day_layout = Layout::horizontal(day_constraints).split(day_area);
        
        for (day_index, &date) in week.iter().enumerate() {
            if day_index >= day_layout.len() {
                break;
            }
            
            let day_area = day_layout[day_index];
            render_day_cell(frame, day_area, date, month_view, tasks);
        }
    }
}


fn render_day_cell(
    frame: &mut Frame,
    area: Rect,
    date: NaiveDate,
    month_view: &MonthView,
    tasks: &[Task],
) {
    let is_current_month = date.month() == month_view.current_date.month();
    let is_selected_day = matches!(month_view.selection.selection_type, SelectionType::Day(selected_date) if selected_date == date);
    
    // Get tasks for this day
    let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(date)).collect();
    
    // Day style
    let day_style = if is_selected_day {
        Style::default().bg(Color::Blue).fg(Color::White)
    } else if !is_current_month {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };
    
    let border_style = if is_selected_day {
        Style::default().fg(Color::Blue)
    } else {
        Style::default().fg(Color::Gray)
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style);
    
    let inner_area = block.inner(area);
    frame.render_widget(block, area);
    
    // Always render day number, even if space is very limited
    let day_number = format!("{}", date.day());
    let day_paragraph = Paragraph::new(day_number).style(day_style);
    
    if inner_area.height == 0 || inner_area.width == 0 {
        // If no inner space, just return - the border should still be visible
        return;
    }

    // Split inner area for day number and tasks
    let day_layout = Layout::vertical([
        Constraint::Length(1),  // Day number - always exactly 1 line
        Constraint::Min(0),     // Tasks - expand to fill remaining space, can be 0 if no space
    ]).split(inner_area);
    
    // Always render the day number if we have any space
    if day_layout.len() > 0 && day_layout[0].height > 0 {
        frame.render_widget(day_paragraph, day_layout[0]);
    }
    
    // Render tasks if there are any and if we have space for them
    if !day_tasks.is_empty() && day_layout.len() > 1 && day_layout[1].height > 0 {
        let task_items: Vec<ListItem> = day_tasks
            .iter()
            .enumerate()
            .map(|(_index, task)| {
                let is_selected_task = matches!(
                    month_view.selection.selection_type,
                    SelectionType::Task(ref task_id) if task_id == &task.id
                );
                
                let style = if is_selected_task {
                    Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD)
                } else if task.completed {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                };
                
                let title = if task.title.len() > 8 {
                    format!("{}...", &task.title[..5])
                } else {
                    task.title.clone()
                };
                
                ListItem::new(title).style(style)
            })
            .collect();
        
        let task_list = List::new(task_items)
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(task_list, day_layout[1]);
    }
}
