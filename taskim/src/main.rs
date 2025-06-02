mod task;
mod month_view;
mod task_edit;
mod data;

use crate::month_view::{MonthView, render_month_view, SelectionType};
use crate::task::TaskData;
use crate::task_edit::{TaskEditState, render_task_edit_popup};
use crate::data::{load_data, save_data};

use chrono::Local;
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
    ConfirmDelete(String), // task id
}

struct App {
    mode: AppMode,
    data: TaskData,
    month_view: MonthView,
    should_exit: bool,
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
                        self.data.events.push(task);
                    } else {
                        // Update existing task
                        if let Some(existing) = self.data.events.iter_mut().find(|t| Some(&t.id) == new_state.task_id.as_ref()) {
                            *existing = task;
                        }
                    }
                    self.mode = AppMode::Normal;
                    self.save()?;
                } else {
                    self.mode = AppMode::TaskEdit(new_state);
                }
            }
            AppMode::ConfirmDelete(task_id) => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        // Delete the task
                        self.data.events.retain(|t| &t.id != task_id);
                        self.mode = AppMode::Normal;
                        self.save()?;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        self.mode = AppMode::Normal;
                    }
                    _ => {}
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
                if let Some(task_id) = self.month_view.get_selected_task_id() {
                    self.mode = AppMode::ConfirmDelete(task_id);
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
            AppMode::ConfirmDelete(_) => {
                self.render_delete_confirmation(frame, area);
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
                vec![
                    Line::from(vec![
                        Span::styled("hjkl", Style::default().fg(Color::Green)),
                        Span::raw(": Move | "),
                        Span::styled("i", Style::default().fg(Color::Green)),
                        Span::raw(": Insert/Edit | "),
                        Span::styled("x", Style::default().fg(Color::Red)),
                        Span::raw(": Delete | "),
                        Span::styled("c", Style::default().fg(Color::Blue)),
                        Span::raw(": Toggle Complete | "),
                        Span::styled("n/p", Style::default().fg(Color::Yellow)),
                        Span::raw(": Month | "),
                        Span::styled("N/P", Style::default().fg(Color::Yellow)),
                        Span::raw(": Year | "),
                        Span::styled("q", Style::default().fg(Color::Red)),
                        Span::raw(": Quit"),
                    ])
                ]
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
            AppMode::ConfirmDelete(_) => {
                vec![
                    Line::from(vec![
                        Span::styled("y", Style::default().fg(Color::Red)),
                        Span::raw(": Yes, delete | "),
                        Span::styled("n", Style::default().fg(Color::Green)),
                        Span::raw(": No, cancel | "),
                        Span::styled("Esc", Style::default().fg(Color::Gray)),
                        Span::raw(": Cancel"),
                    ])
                ]
            }
        };
        
        let footer = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(footer, area);
    }
    
    fn render_delete_confirmation(&self, frame: &mut Frame, area: Rect) {
        // Calculate popup area (centered, 40% width, 20% height)
        let popup_area = centered_rect(40, 20, area);
        
        // Clear the area
        frame.render_widget(ratatui::widgets::Clear, popup_area);
        
        let block = Block::default()
            .title("Delete Task")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red).bg(Color::Black));
        
        let inner_area = block.inner(popup_area);
        frame.render_widget(block, popup_area);
        
        let text = vec![
            Line::from("Are you sure you want to delete this task?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("y", Style::default().fg(Color::Red)),
                Span::raw(": Yes | "),
                Span::styled("n", Style::default().fg(Color::Green)),
                Span::raw(": No"),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(paragraph, inner_area);
    }
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ]).split(r);
    
    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]).split(popup_layout[1])[1]
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}
