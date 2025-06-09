use crate::task::Task;
use crate::utils::days_in_month;
use chrono::{Datelike, NaiveDate};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionType {
    Day(NaiveDate),
    Task(String), // task id
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
        let selection = Self::create_day_selection(current_date);
        
        Self {
            current_date,
            selection,
            weeks,
        }
    }
    
    // Helper method to create a day selection
    fn create_day_selection(date: NaiveDate) -> Selection {
        Selection {
            selection_type: SelectionType::Day(date),
            task_index_in_day: None,
        }
    }
    
    // Helper method to create a task selection
    fn create_task_selection(task_id: String, index: Option<usize>) -> Selection {
        Selection {
            selection_type: SelectionType::Task(task_id),
            task_index_in_day: index,
        }
    }
    
    // Helper method to select a day
    fn select_day(&mut self, date: NaiveDate) {
        self.selection = Self::create_day_selection(date);
    }
    
    // Helper method to select a task
    fn select_task(&mut self, task_id: String, index: Option<usize>) {
        self.selection = Self::create_task_selection(task_id, index);
    }
    
    // Helper method to transition to a new month and update everything
    fn transition_to_month(&mut self, new_date: NaiveDate) {
        self.current_date = new_date;
        self.weeks = Self::build_weeks(self.current_date);
        self.select_day(self.current_date);
    }
    
    // Helper method to navigate to a date, handling month transitions if needed
    fn navigate_to_date(&mut self, target_date: NaiveDate) {
        // Check if we need to change months
        if target_date.month() != self.current_date.month() || target_date.year() != self.current_date.year() {
            self.current_date = target_date.with_day(1).unwrap();
            self.weeks = Self::build_weeks(self.current_date);
        }
        self.select_day(target_date);
    }
    
    // Public method to rebuild weeks for a given date
    pub fn build_weeks_for_date(date: NaiveDate) -> Vec<Vec<NaiveDate>> {
        Self::build_weeks(date)
    }
    
    fn build_weeks(date: NaiveDate) -> Vec<Vec<NaiveDate>> {
        let first_of_month = date.with_day(1).unwrap();
        let last_of_month = date.with_day(
            days_in_month(date.year(), date.month())
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
    

    
    pub fn prev_month(&mut self) {
        let new_date = if self.current_date.month() == 1 {
            NaiveDate::from_ymd_opt(self.current_date.year() - 1, 12, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month() - 1, 1).unwrap()
        };
        self.transition_to_month(new_date);
    }
    
    pub fn next_year(&mut self) {
        let new_date = NaiveDate::from_ymd_opt(self.current_date.year() + 1, self.current_date.month(), 1).unwrap();
        self.transition_to_month(new_date);
    }
    
    pub fn prev_year(&mut self) {
        let new_date = NaiveDate::from_ymd_opt(self.current_date.year() - 1, self.current_date.month(), 1).unwrap();
        self.transition_to_month(new_date);
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
                        let days_in_month = days_in_month(self.current_date.year(), self.current_date.month());
                        let safe_day = std::cmp::min(target_day, days_in_month);
                        if let Some(target_date) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), safe_day) {
                            self.select_day(target_date);
                            
                            // Auto-select first task if available
                            let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(target_date)).collect();
                            if !day_tasks.is_empty() {
                                let mut sorted_tasks = day_tasks;
                                sorted_tasks.sort_by_key(|t| t.order);
                                self.select_task(sorted_tasks[0].id.clone(), Some(0));
                            }
                        }
                    } else {
                        self.select_day(new_date);
                        
                        // Auto-select first task if available
                        let day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(new_date)).collect();
                        if !day_tasks.is_empty() {
                            let mut sorted_tasks = day_tasks;
                            sorted_tasks.sort_by_key(|t| t.order);
                            self.select_task(sorted_tasks[0].id.clone(), Some(0));
                        }
                    }
                }
            }
            SelectionType::Task(task_id) => {
                let task_id = task_id.clone();
                // Find the current task and move to previous task in the same day
                if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                    let task_date = task.start.date_naive();
                    let mut day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(task_date)).collect();
                    day_tasks.sort_by_key(|t| t.order); // Sort by order
                    
                    if let Some(current_index) = day_tasks.iter().position(|t| t.id == task_id) {
                        if current_index > 0 {
                            // Move to previous task
                            let prev_task = &day_tasks[current_index - 1];
                            self.select_task(prev_task.id.clone(), Some(current_index - 1));
                        } else {
                            // Move to day selection
                            self.select_day(task_date);
                        }
                    }
                }
            }
        }
    }
    
    pub fn move_down(&mut self, tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                let current_date = *date;
                // Check if there are tasks on this day
                let mut day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(current_date)).collect();
                day_tasks.sort_by_key(|t| t.order); // Sort by order
                
                if !day_tasks.is_empty() {
                    // Move to first task (ordered)
                    self.select_task(day_tasks[0].id.clone(), Some(0));
                } else {
                    // Move down one week
                    if let Some(new_date) = current_date.checked_add_signed(chrono::Duration::weeks(1)) {
                        self.navigate_to_date(new_date);
                        
                        // After navigating, check if the new day has tasks and auto-select first task
                        let new_day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(new_date)).collect();
                        if !new_day_tasks.is_empty() {
                            let mut sorted_tasks = new_day_tasks;
                            sorted_tasks.sort_by_key(|t| t.order);
                            self.select_task(sorted_tasks[0].id.clone(), Some(0));
                        }
                    }
                }
            }
            SelectionType::Task(task_id) => {
                let task_id = task_id.clone();
                // Find the current task and move to next task in the same day or to next week
                if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                    let task_date = task.start.date_naive();
                    let mut day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(task_date)).collect();
                    day_tasks.sort_by_key(|t| t.order); // Sort by order
                    
                    if let Some(current_index) = day_tasks.iter().position(|t| t.id == task_id) {
                        if current_index < day_tasks.len() - 1 {
                            // Move to next task
                            let next_task = &day_tasks[current_index + 1];
                            self.select_task(next_task.id.clone(), Some(current_index + 1));
                        } else {
                            // Move to next week same day
                            if let Some(new_date) = task_date.checked_add_signed(chrono::Duration::weeks(1)) {
                                self.navigate_to_date(new_date);
                                
                                // Check if new day has tasks and auto-select first task
                                let new_day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(new_date)).collect();
                                if !new_day_tasks.is_empty() {
                                    let mut sorted_tasks = new_day_tasks;
                                    sorted_tasks.sort_by_key(|t| t.order);
                                    self.select_task(sorted_tasks[0].id.clone(), Some(0));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn move_left(&mut self, _tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                if let Some(new_date) = date.checked_sub_signed(chrono::Duration::days(1)) {
                    self.navigate_to_date(new_date);
                }
            }
            SelectionType::Task(task_id) => {
                // Move to the day containing this task
                if let Some(task) = _tasks.iter().find(|t| &t.id == task_id) {
                    let task_date = task.start.date_naive();
                    self.select_day(task_date);
                }
            }
        }
    }
    
    pub fn move_right(&mut self, _tasks: &[Task]) {
        match &self.selection.selection_type {
            SelectionType::Day(date) => {
                if let Some(new_date) = date.checked_add_signed(chrono::Duration::days(1)) {
                    self.navigate_to_date(new_date);
                }
            }
            SelectionType::Task(task_id) => {
                // Move to the day containing this task
                if let Some(task) = _tasks.iter().find(|t| &t.id == task_id) {
                    let task_date = task.start.date_naive();
                    self.select_day(task_date);
                }
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
    pub fn get_selected_date(&self, tasks: &[Task]) -> NaiveDate {
        match &self.selection.selection_type {
            SelectionType::Day(date) => *date,
            SelectionType::Task(task_id) => {
                // Find the actual task and return its date
                if let Some(task) = tasks.iter().find(|t| &t.id == task_id) {
                    task.start.date_naive()
                } else {
                    // Fallback to current date if task not found
                    self.current_date
                }
            }
        }
    }

    // Move to next week (same day of week)
    pub fn next_week(&mut self, tasks: &[Task]) {
        let current_selected = self.get_selected_date(tasks);
        if let Some(new_date) = current_selected.checked_add_signed(chrono::Duration::weeks(1)) {
            self.navigate_to_date(new_date);
        }
    }

    // Move to previous week (same day of week)
    pub fn prev_week(&mut self, tasks: &[Task]) {
        let current_selected = self.get_selected_date(tasks);
        if let Some(new_date) = current_selected.checked_sub_signed(chrono::Duration::weeks(1)) {
            self.navigate_to_date(new_date);
        }
    }

    // Move to first day of current month
    pub fn first_day_of_month(&mut self) {
        let first_day = self.current_date.with_day(1).unwrap();
        self.select_day(first_day);
    }

    // Move to last day of current month
    pub fn last_day_of_month(&mut self) {
        let days_in_month = days_in_month(self.current_date.year(), self.current_date.month());
        
        if let Some(last_day) = NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), days_in_month) {
            self.select_day(last_day);
        }
    }

    // Helper method to preserve day when changing months
    fn navigate_to_month_preserve_day(&mut self, new_year: i32, new_month: u32) {
        // Get the currently selected date from the selection
        let current_selected = match &self.selection.selection_type {
            SelectionType::Day(date) => *date,
            SelectionType::Task(_) => {
                // For task selections, we'll use the current date as fallback
                // since we don't have task data here
                self.current_date
            }
        };
        let target_day = current_selected.day();
        
        // Calculate days in the target month
        let days_in_month = days_in_month(new_year, new_month);
        
        // Preserve day or use last day of month if target day doesn't exist
        let safe_day = std::cmp::min(target_day, days_in_month);
        
        self.current_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).unwrap();
        self.weeks = Self::build_weeks(self.current_date);
        
        if let Some(target_date) = NaiveDate::from_ymd_opt(new_year, new_month, safe_day) {
            self.select_day(target_date);
        }
    }

    // Navigate to previous month while preserving the current day when possible
    pub fn prev_month_preserve_day(&mut self) {
        let (new_year, new_month) = if self.current_date.month() == 1 {
            (self.current_date.year() - 1, 12)
        } else {
            (self.current_date.year(), self.current_date.month() - 1)
        };
        
        self.navigate_to_month_preserve_day(new_year, new_month);
    }

    // Navigate to next month while preserving the current day when possible
    pub fn next_month_preserve_day(&mut self) {
        let (new_year, new_month) = if self.current_date.month() == 12 {
            (self.current_date.year() + 1, 1)
        } else {
            (self.current_date.year(), self.current_date.month() + 1)
        };
        
        self.navigate_to_month_preserve_day(new_year, new_month);
    }

    // Navigate to today's date
    pub fn go_to_today(&mut self) {
        use chrono::Local;
        
        let today = Local::now().date_naive();
        self.navigate_to_date(today);
    }
    
    // Helper method to get the current task's order within its day
    pub fn get_current_task_order(&self, tasks: &[Task]) -> Option<u32> {
        match &self.selection.selection_type {
            SelectionType::Task(task_id) => {
                tasks.iter().find(|t| &t.id == task_id).map(|t| t.order)
            }
            _ => None,
        }
    }
    
    // Helper method to select a task by its order within a day
    pub fn select_task_by_order(&mut self, date: NaiveDate, order: u32, tasks: &[Task]) {
        let mut day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(date)).collect();
        day_tasks.sort_by_key(|t| t.order);
        
        if let Some(task) = day_tasks.iter().find(|t| t.order == order) {
            let index = day_tasks.iter().position(|t| t.id == task.id);
            self.select_task(task.id.clone(), index);
        }
    }
}

pub fn render_month_view(
    frame: &mut Frame,
    area: Rect,
    month_view: &MonthView,
    tasks: &[Task],
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
    
    // Calculate constraints for each week based on max tasks, ensuring proper expansion
    let week_constraints: Vec<Constraint> = month_view.weeks.iter()
        .map(|week| {
            // Find the maximum number of tasks in any day of this week
            let max_tasks_in_week = week.iter()
                .map(|&date| tasks.iter().filter(|t| t.is_on_date(date)).count())
                .max()
                .unwrap_or(0);
            
            // Calculate proper height: day_number(1) + tasks(N) + borders(2) + padding(1)
            let week_height = if max_tasks_in_week == 0 {
                4 // Minimum height when no tasks: day + borders + padding
            } else {
                1 + max_tasks_in_week + 3 // day_number(1) + tasks(N) + borders+padding(3)
            };
            
            Constraint::Length(week_height as u16)
        })
        .collect();
    
    let week_layout = Layout::vertical(week_constraints).split(inner_area);
    
    for (week_index, week) in month_view.weeks.iter().enumerate() {
        if week_index >= week_layout.len() {
            break;
        }
        
        let week_area = week_layout[week_index];
        
        // Render days directly
        let day_constraints: Vec<Constraint> = (0..7)
            .map(|_| Constraint::Percentage(100 / 7))
            .collect();
        
        let day_layout = Layout::horizontal(day_constraints).split(week_area);
        
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
    
    // Get tasks for this day, sorted by order
    let mut day_tasks: Vec<_> = tasks.iter().filter(|t| t.is_on_date(date)).collect();
    day_tasks.sort_by_key(|t| t.order);
    
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
    
    // Always render day number, ensuring it gets proper space
    let day_number = format!("{}", date.day());
    let day_paragraph = Paragraph::new(day_number).style(day_style);
    
    if inner_area.height == 0 || inner_area.width == 0 {
        // If no inner space, just return - the border should still be visible
        return;
    }

    // FIXED: Day number gets top line, tasks get remaining space if available
    if day_tasks.is_empty() {
        // No tasks: just render day number in available space
        frame.render_widget(day_paragraph, inner_area);
    } else {
        // With tasks: day number gets exactly 1 line at top, tasks get rest
        let day_layout = Layout::vertical([
            Constraint::Length(1),                      // Day number - exactly 1 line
            Constraint::Min(1),                         // Tasks - all remaining space
        ]).split(inner_area);
        
        // Render day number in top line
        if day_layout.len() > 0 && day_layout[0].height > 0 {
            frame.render_widget(day_paragraph, day_layout[0]);
        }
        
        // Render tasks in remaining space
        if day_layout.len() > 1 && day_layout[1].height > 0 {
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
}
