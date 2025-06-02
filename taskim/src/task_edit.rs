use crate::task::Task;
use chrono::NaiveDate;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TaskEditState {
    pub task_id: Option<String>,
    pub title: String,
    pub content: String,
    pub editing_field: EditingField,
    pub is_new_task: bool,
    pub date: NaiveDate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditingField {
    Title,
    Content,
}

impl TaskEditState {
    pub fn new_task(date: NaiveDate) -> Self {
        Self {
            task_id: None,
            title: String::new(),
            content: String::new(),
            editing_field: EditingField::Title,
            is_new_task: true,
            date,
        }
    }
    
    pub fn edit_task(task: &Task) -> Self {
        let content = task.comments.first()
            .map(|c| c.text.clone())
            .unwrap_or_default();
            
        Self {
            task_id: Some(task.id.clone()),
            title: task.title.clone(),
            content,
            editing_field: EditingField::Title,
            is_new_task: false,
            date: task.start.date_naive(),
        }
    }
    
    pub fn add_char(&mut self, ch: char) {
        match self.editing_field {
            EditingField::Title => self.title.push(ch),
            EditingField::Content => self.content.push(ch),
        }
    }
    
    pub fn remove_char(&mut self) {
        match self.editing_field {
            EditingField::Title => { self.title.pop(); },
            EditingField::Content => { self.content.pop(); },
        }
    }
    
    pub fn switch_field(&mut self) {
        self.editing_field = match self.editing_field {
            EditingField::Title => EditingField::Content,
            EditingField::Content => EditingField::Title,
        };
    }
    
    pub fn to_task(&self) -> Task {
        let start = self.date.and_hms_opt(9, 0, 0).unwrap()
            .and_local_timezone(chrono::Local)
            .single()
            .unwrap()
            .to_utc();
            
        let mut task = Task::new(self.title.clone(), start);
        
        if !self.content.is_empty() {
            task.add_comment(self.content.clone());
        }
        
        if let Some(ref task_id) = self.task_id {
            task.id = task_id.clone();
        }
        
        task
    }
}

pub fn render_task_edit_popup(
    frame: &mut Frame,
    area: Rect,
    state: &TaskEditState,
) {
    // Calculate popup area (centered, 60% width, 40% height)
    let popup_area = centered_rect(60, 40, area);
    
    // Clear the area
    frame.render_widget(Clear, popup_area);
    
    // Create the block
    let title = if state.is_new_task { "New Task" } else { "Edit Task" };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    
    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);
    
    // Split the inner area for title, content, and instructions
    let layout = Layout::vertical([
        Constraint::Length(3), // Title field
        Constraint::Min(3),    // Content field
        Constraint::Length(2), // Instructions
    ]).split(inner_area);
    
    // Render title field
    let title_style = if state.editing_field == EditingField::Title {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let title_block = Block::default()
        .title("Title")
        .borders(Borders::ALL)
        .border_style(title_style);
    
    let title_paragraph = Paragraph::new(state.title.as_str())
        .block(title_block)
        .style(title_style);
    
    frame.render_widget(title_paragraph, layout[0]);
    
    // Render content field
    let content_style = if state.editing_field == EditingField::Content {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let content_block = Block::default()
        .title("Content")
        .borders(Borders::ALL)
        .border_style(content_style);
    
    let content_paragraph = Paragraph::new(state.content.as_str())
        .block(content_block)
        .style(content_style)
        .wrap(Wrap { trim: true });
    
    frame.render_widget(content_paragraph, layout[1]);
    
    // Render instructions
    let instructions = vec![
        Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Green)),
            Span::raw(": Switch field | "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(": Save | "),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(": Cancel"),
        ])
    ];
    
    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray));
    
    frame.render_widget(instructions_paragraph, layout[2]);
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
