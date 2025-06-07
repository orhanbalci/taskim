mod task;
mod month_view;
mod task_edit;
mod data;
mod undo;
mod config;

use crate::month_view::{MonthView, render_month_view, SelectionType};
use crate::task::TaskData;
use crate::task_edit::{TaskEditState, render_task_edit_popup};
use crate::data::{load_data, save_data};
use crate::undo::{UndoStack, Operation};
use crate::config::KEYBINDINGS;

use chrono::{Local, Timelike};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Clone, PartialEq)]
enum AppMode {
    Normal,
    TaskEdit(TaskEditState),
}

struct App {
    mode: AppMode,
    data: TaskData,
    month_view: MonthView,
    should_exit: bool,
    undo_stack: UndoStack,
    yanked_task: Option<crate::task::Task>, // Store yanked task for paste operation
}

impl App {
    fn new() -> Self {
        let data = load_data();
        let current_date = Local::now().date_naive();
        let month_view = MonthView::new(current_date);
        
        Self {
            mode: AppMode::Normal,
            data,
            month_view,
            should_exit: false,
            undo_stack: UndoStack::new(50), // Allow up to 50 undo operations
            yanked_task: None,
        }
    }
    
    fn save(&self) -> Result<()> {
        save_data(&self.data).map_err(|e| color_eyre::eyre::eyre!(e))?;
        Ok(())
    }
    
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match &self.mode {
            AppMode::Normal => self.handle_normal_mode_key(key)?,
            AppMode::TaskEdit(state) => {
                let mut new_state = state.clone();
                if self.handle_task_edit_key(key, &mut new_state)? {
                    // Task edit completed
                    let task = new_state.to_task();
                    if new_state.is_new_task {
                        // Track task creation
                        self.undo_stack.push(Operation::CreateTask {
                            task: task.clone(),
                        });
                        self.data.events.push(task);
                    } else {
                        // Track task edit
                        if let Some(existing) = self.data.events.iter_mut().find(|t| Some(&t.id) == new_state.task_id.as_ref()) {
                            let old_task = existing.clone();
                            *existing = task.clone();
                            
                            self.undo_stack.push(Operation::EditTask {
                                task_id: task.id.clone(),
                                old_task,
                                new_task: task,
                            });
                        }
                    }
                    self.mode = AppMode::Normal;
                    self.save()?;
                } else {
                    self.mode = AppMode::TaskEdit(new_state);
                }
            }
        }
        Ok(())
    }
    
    fn handle_normal_mode_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        // Handle keybindings
        if KEYBINDINGS.force_quit.matches(key.code, key.modifiers) {
            self.should_exit = true;
            return Ok(());
        }
        
        if KEYBINDINGS.quit.matches(key.code, key.modifiers) || 
           KEYBINDINGS.quit_alt.matches(key.code, key.modifiers) {
            self.should_exit = true;
        } else if KEYBINDINGS.move_left.matches(key.code, key.modifiers) {
            self.month_view.move_left(&self.data.events);
        } else if KEYBINDINGS.move_down.matches(key.code, key.modifiers) {
            self.month_view.move_down(&self.data.events);
        } else if KEYBINDINGS.move_up.matches(key.code, key.modifiers) {
            self.month_view.move_up(&self.data.events);
        } else if KEYBINDINGS.move_right.matches(key.code, key.modifiers) {
            self.month_view.move_right(&self.data.events);
        } else if KEYBINDINGS.insert_edit.matches(key.code, key.modifiers) {
            match &self.month_view.selection.selection_type {
                SelectionType::Day(date) => {
                    // Create new task
                    let edit_state = TaskEditState::new_task(*date);
                    self.mode = AppMode::TaskEdit(edit_state);
                }
                SelectionType::Task(task_id) => {
                    // Edit existing task
                    if let Some(task) = self.data.events.iter().find(|t| &t.id == task_id) {
                        let edit_state = TaskEditState::edit_task(task);
                        self.mode = AppMode::TaskEdit(edit_state);
                    }
                }
                SelectionType::WeekGoal(_) => {
                    // TODO: Edit weekly goal
                }
            }
        } else if KEYBINDINGS.delete.matches(key.code, key.modifiers) {
            // Immediately delete the selected task
            if let Some(task_id) = self.month_view.get_selected_task_id() {
                if let Some(task_index) = self.data.events.iter().position(|t| t.id == task_id) {
                    let deleted_task = self.data.events.remove(task_index);
                    let task_date = deleted_task.start.date_naive();
                    
                    // Track deletion for undo functionality
                    self.undo_stack.push(Operation::DeleteTask {
                        task: deleted_task,
                        original_date: task_date,
                    });
                    
                    // Check if there are any remaining tasks on the same date
                    let remaining_tasks: Vec<_> = self.data.events.iter()
                        .filter(|t| t.is_on_date(task_date))
                        .collect();
                    
                    if remaining_tasks.is_empty() {
                        // No more tasks on this day, select the day itself
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Day(task_date),
                            task_index_in_day: None,
                        };
                    } else {
                        // Select the first remaining task
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Task(remaining_tasks[0].id.clone()),
                            task_index_in_day: Some(0),
                        };
                    }
                    
                    self.save()?;
                }
            }
        } else if KEYBINDINGS.undo.matches(key.code, key.modifiers) {
            // Undo last operation
            if let Some(operation) = self.undo_stack.undo() {
                match operation {
                    Operation::DeleteTask { task, original_date: _ } => {
                        // Restore deleted task
                        self.data.events.push(task.clone());
                        
                        // Select the restored task
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Task(task.id),
                            task_index_in_day: Some(0),
                        };
                    }
                    Operation::EditTask { task_id, old_task, new_task: _ } => {
                        // Revert task edit
                        if let Some(existing) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                            *existing = old_task;
                        }
                    }
                    Operation::CreateTask { task } => {
                        // Remove created task
                        self.data.events.retain(|t| t.id != task.id);
                        
                        // Select the day where the task was
                        let task_date = task.start.date_naive();
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Day(task_date),
                            task_index_in_day: None,
                        };
                    }
                    Operation::YankPaste { task_id, old_date, new_date: _ } => {
                        // TODO: Implement when yank/paste is added
                        // For now, we'll revert the task to its old date
                        if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                            let duration = task.end - task.start;
                            let old_datetime = old_date.and_hms_opt(
                                task.start.time().hour(),
                                task.start.time().minute(),
                                task.start.time().second()
                            ).unwrap().and_utc();
                            task.start = old_datetime;
                            task.end = old_datetime + duration;
                        }
                    }
                }
                self.save()?;
            }
        } else if KEYBINDINGS.redo.matches(key.code, key.modifiers) {
            // Redo last undone operation
            if let Some(operation) = self.undo_stack.redo() {
                match operation {
                    Operation::DeleteTask { task, original_date: _ } => {
                        // Re-delete the task
                        self.data.events.retain(|t| t.id != task.id);
                        
                        // Select the day where the task was
                        let task_date = task.start.date_naive();
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Day(task_date),
                            task_index_in_day: None,
                        };
                    }
                    Operation::EditTask { task_id, old_task: _, new_task } => {
                        // Re-apply task edit
                        if let Some(existing) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                            *existing = new_task;
                        }
                    }
                    Operation::CreateTask { task } => {
                        // Re-create task
                        self.data.events.push(task.clone());
                        
                        // Select the restored task
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Task(task.id),
                            task_index_in_day: Some(0),
                        };
                    }
                    Operation::YankPaste { task_id, old_date: _, new_date } => {
                        // TODO: Implement when yank/paste is added
                        if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                            let duration = task.end - task.start;
                            let new_datetime = new_date.and_hms_opt(
                                task.start.time().hour(),
                                task.start.time().minute(),
                                task.start.time().second()
                            ).unwrap().and_utc();
                            task.start = new_datetime;
                            task.end = new_datetime + duration;
                        }
                    }
                }
                self.save()?;
            }
        } else if KEYBINDINGS.toggle_complete.matches(key.code, key.modifiers) {
            // Toggle task completion
            if let Some(task_id) = self.month_view.get_selected_task_id() {
                if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                    task.completed = !task.completed;
                    self.save()?;
                }
            }
        } else if KEYBINDINGS.yank.matches(key.code, key.modifiers) {
            // Yank (copy) task
            if let Some(task_id) = self.month_view.get_selected_task_id() {
                if let Some(task) = self.data.events.iter().find(|t| t.id == task_id) {
                    self.yanked_task = Some(task.clone());
                }
            }
        } else if KEYBINDINGS.paste.matches(key.code, key.modifiers) {
            // Paste task
            if let Some(yanked_task) = &self.yanked_task {
                let selected_date = self.month_view.get_selected_date();
                let mut new_task = yanked_task.clone();
                
                // Generate new ID for the pasted task
                new_task.id = format!("task_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default());
                
                // Set new start/end times for the selected date
                let duration = new_task.end - new_task.start;
                let new_start = selected_date.and_hms_opt(
                    new_task.start.time().hour(),
                    new_task.start.time().minute(),
                    new_task.start.time().second()
                ).unwrap().and_utc();
                new_task.start = new_start;
                new_task.end = new_start + duration;
                
                // Track the paste operation for undo
                self.undo_stack.push(Operation::CreateTask {
                    task: new_task.clone(),
                });
                
                // Add the task to data
                self.data.events.push(new_task);
                self.save()?;
            }
        } else if KEYBINDINGS.next_month.matches(key.code, key.modifiers) {
            // Next month (vim-style: L)
            self.month_view.next_month();
        } else if KEYBINDINGS.prev_month.matches(key.code, key.modifiers) {
            // Previous month (vim-style: H)
            self.month_view.prev_month();
        } else if KEYBINDINGS.next_year.matches(key.code, key.modifiers) {
            // Next/Last year (vim-style: G)
            self.month_view.next_year();
        } else if KEYBINDINGS.prev_year.matches(key.code, key.modifiers) {
            // Previous/First year (vim-style: gg)
            self.month_view.prev_year();
        } else if KEYBINDINGS.next_week.matches(key.code, key.modifiers) {
            // Next week (vim-style: w)
            self.month_view.next_week();
        } else if KEYBINDINGS.prev_week.matches(key.code, key.modifiers) {
            // Previous week (vim-style: b)
            self.month_view.prev_week();
        } else if KEYBINDINGS.first_day_of_month.matches(key.code, key.modifiers) {
            // First day of month (vim-style: 0)
            self.month_view.first_day_of_month();
        } else if KEYBINDINGS.last_day_of_month.matches(key.code, key.modifiers) {
            // Last day of month (vim-style: $)
            self.month_view.last_day_of_month();
        }
        Ok(())
    }
    
    fn handle_task_edit_key(&mut self, key: crossterm::event::KeyEvent, state: &mut TaskEditState) -> Result<bool> {
        if KEYBINDINGS.cancel_edit.matches(key.code, key.modifiers) {
            // Cancel edit
            return Ok(true);
        } else if KEYBINDINGS.save_task.matches(key.code, key.modifiers) {
            // Save task
            if !state.title.trim().is_empty() {
                return Ok(true);
            }
        } else if KEYBINDINGS.switch_field.matches(key.code, key.modifiers) {
            state.switch_field();
        } else if KEYBINDINGS.backspace.matches(key.code, key.modifiers) {
            state.remove_char();
        } else if let KeyCode::Char(ch) = key.code {
            state.add_char(ch);
        }
        Ok(false)
    }
    
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            
            if self.should_exit {
                break;
            }
            
            if let Ok(event) = event::read() {
                if let Event::Key(key_event) = event {
                    self.handle_key_event(key_event)?;
                }
            }
        }
        Ok(())
    }
    
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        
        // Create main layout
        let layout = Layout::vertical([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(2), // Footer
        ]).split(area);
        
        // Render header
        self.render_header(frame, layout[0]);
        
        // Render main content
        render_month_view(frame, layout[1], &self.month_view, &self.data.events, &self.data.weekly_goals);
        
        // Render footer
        self.render_footer(frame, layout[2]);
        
        // Render mode-specific overlays
        match &self.mode {
            AppMode::TaskEdit(state) => {
                render_task_edit_popup(frame, area, state);
            }
            AppMode::Normal => {}
        }
    }
    
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = "Task Manager - Month View";
        let header = Paragraph::new(title)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(header, area);
    }
    
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let help_text = match &self.mode {
            AppMode::Normal => {
                let spans = KEYBINDINGS.get_normal_mode_help_spans(
                    self.undo_stack.can_undo(),
                    self.undo_stack.can_redo()
                );
                vec![Line::from(spans)]
            }
            AppMode::TaskEdit(_) => {
                let spans = KEYBINDINGS.get_edit_mode_help_spans();
                vec![Line::from(spans)]
            }
        };
        
        let footer = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(footer, area);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}
