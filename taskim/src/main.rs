mod task;
mod month_view;
mod task_edit;
mod data;
mod undo;

use crate::month_view::{MonthView, render_month_view, SelectionType};
use crate::task::{TaskData, Task};
use crate::task_edit::{TaskEditState, render_task_edit_popup};
use crate::data::{load_data, save_data};
use crate::undo::{UndoStack, Operation};

use chrono::{Local, Timelike};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
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
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => self.should_exit = true,
                _ => {}
            }
            return Ok(());
        }
        
        // Handle space + key combinations for month/year navigation
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            match key.code {
                KeyCode::Char(' ') => {
                    // Wait for next key for space combinations
                    // For now, we'll handle them as individual keys
                }
                _ => {}
            }
        }
        
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') => self.month_view.move_left(&self.data.events),
            KeyCode::Char('j') => self.month_view.move_down(&self.data.events),
            KeyCode::Char('k') => self.month_view.move_up(&self.data.events),
            KeyCode::Char('l') => self.month_view.move_right(&self.data.events),
            KeyCode::Char('i') => {
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
            }
            KeyCode::Char('x') => {
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
            }
            KeyCode::Char('u') => {
                // Undo last operation
                if let Some(operation) = self.undo_stack.pop() {
                    match operation {
                        Operation::DeleteTask { task, original_date } => {
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
                        Operation::YankPaste { task_id, old_date, new_date } => {
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
            }
            KeyCode::Char('c') => {
                // Toggle task completion
                if let Some(task_id) = self.month_view.get_selected_task_id() {
                    if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                        task.completed = !task.completed;
                        self.save()?;
                    }
                }
            }
            KeyCode::Char('n') => {
                // Next month
                self.month_view.next_month();
            }
            KeyCode::Char('p') => {
                // Previous month
                self.month_view.prev_month();
            }
            KeyCode::Char('N') => {
                // Next year
                self.month_view.next_year();
            }
            KeyCode::Char('P') => {
                // Previous year
                self.month_view.prev_year();
            }
            _ => {}
        }
        Ok(())
    }
    
    fn handle_task_edit_key(&mut self, key: crossterm::event::KeyEvent, state: &mut TaskEditState) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                // Cancel edit
                return Ok(true);
            }
            KeyCode::Enter => {
                // Save task
                if !state.title.trim().is_empty() {
                    return Ok(true);
                }
            }
            KeyCode::Tab => {
                state.switch_field();
            }
            KeyCode::Backspace => {
                state.remove_char();
            }
            KeyCode::Char(ch) => {
                state.add_char(ch);
            }
            _ => {}
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
                let mut spans = vec![
                    Span::styled("hjkl", Style::default().fg(Color::Green)),
                    Span::raw(": Move | "),
                    Span::styled("i", Style::default().fg(Color::Green)),
                    Span::raw(": Insert/Edit | "),
                    Span::styled("x", Style::default().fg(Color::Red)),
                    Span::raw(": Delete | "),
                ];
                
                // Add undo option if there are operations to undo
                if !self.undo_stack.is_empty() {
                    spans.extend(vec![
                        Span::styled("u", Style::default().fg(Color::Yellow)),
                        Span::raw(": Undo | "),
                    ]);
                }
                
                spans.extend(vec![
                    Span::styled("c", Style::default().fg(Color::Blue)),
                    Span::raw(": Toggle Complete | "),
                    Span::styled("n/p", Style::default().fg(Color::Yellow)),
                    Span::raw(": Month | "),
                    Span::styled("N/P", Style::default().fg(Color::Yellow)),
                    Span::raw(": Year | "),
                    Span::styled("q", Style::default().fg(Color::Red)),
                    Span::raw(": Quit"),
                ]);
                
                vec![Line::from(spans)]
            }
            AppMode::TaskEdit(_) => {
                vec![
                    Line::from(vec![
                        Span::styled("Tab", Style::default().fg(Color::Green)),
                        Span::raw(": Switch field | "),
                        Span::styled("Enter", Style::default().fg(Color::Green)),
                        Span::raw(": Save | "),
                        Span::styled("Esc", Style::default().fg(Color::Red)),
                        Span::raw(": Cancel"),
                    ])
                ]
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
