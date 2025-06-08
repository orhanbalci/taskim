// Taskim Configuration
// Edit this file to customize your keybindings

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
    pub description: &'static str,
    pub color: Color,
}

impl KeyBinding {
    pub fn matches(&self, key: KeyCode, modifiers: KeyModifiers) -> bool {
        self.key == key && self.modifiers == modifiers
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    // Navigation
    pub move_left: KeyBinding,
    pub move_down: KeyBinding,
    pub move_up: KeyBinding,
    pub move_right: KeyBinding,
    
    // Task operations
    pub insert_edit: KeyBinding,
    pub delete: KeyBinding,
    pub toggle_complete: KeyBinding,
    pub yank: KeyBinding,
    pub paste: KeyBinding,
    
    // Undo/Redo
    pub undo: KeyBinding,
    pub redo: KeyBinding,
    
    // Month/Year navigation (vim-style)
    pub next_month: KeyBinding,
    pub prev_month: KeyBinding,
    pub next_year: KeyBinding,
    pub prev_year: KeyBinding,
    
    // Week navigation (vim-style)
    pub next_week: KeyBinding,
    pub prev_week: KeyBinding,
    
    // Day navigation (vim-style)
    pub first_day_of_month: KeyBinding,
    pub last_day_of_month: KeyBinding,
    
    // Go to today
    pub go_to_today: KeyBinding,
    
    // Task editing
    pub save_task: KeyBinding,
    pub cancel_edit: KeyBinding,
    pub switch_field: KeyBinding,
    pub backspace: KeyBinding,
    
    // App control
    pub quit: KeyBinding,
    pub quit_alt: KeyBinding,
    pub force_quit: KeyBinding,
}

// ============================================================================
// CUSTOMIZE YOUR KEYBINDINGS HERE
// ============================================================================

pub const KEYBINDINGS: Config = Config {
    // Navigation (vim-style by default)
    move_left: KeyBinding { key: KeyCode::Char('h'), modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
    move_down: KeyBinding { key: KeyCode::Char('j'), modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
    move_up: KeyBinding { key: KeyCode::Char('k'), modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
    move_right: KeyBinding { key: KeyCode::Char('l'), modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
    
    // Task operations
    insert_edit: KeyBinding { key: KeyCode::Char('i'), modifiers: KeyModifiers::NONE, description: "Insert/Edit", color: Color::Green },
    delete: KeyBinding { key: KeyCode::Char('x'), modifiers: KeyModifiers::NONE, description: "Delete", color: Color::Red },
    toggle_complete: KeyBinding { key: KeyCode::Char('c'), modifiers: KeyModifiers::NONE, description: "Toggle Complete", color: Color::Blue },
    
    // Yank/Paste (vim-style)
    yank: KeyBinding { key: KeyCode::Char('y'), modifiers: KeyModifiers::NONE, description: "Yank (Copy)", color: Color::Yellow },
    paste: KeyBinding { key: KeyCode::Char('p'), modifiers: KeyModifiers::NONE, description: "Paste", color: Color::Yellow },
    
    // Undo/Redo
    undo: KeyBinding { key: KeyCode::Char('u'), modifiers: KeyModifiers::NONE, description: "Undo", color: Color::Magenta },
    redo: KeyBinding { key: KeyCode::Char('r'), modifiers: KeyModifiers::CONTROL, description: "Redo", color: Color::Magenta },
    
    // Month/Year navigation (vim-style)
    next_month: KeyBinding { key: KeyCode::Char('L'), modifiers: KeyModifiers::SHIFT, description: "Next Month", color: Color::Cyan },
    prev_month: KeyBinding { key: KeyCode::Char('H'), modifiers: KeyModifiers::SHIFT, description: "Prev Month", color: Color::Cyan },
    next_year: KeyBinding { key: KeyCode::Char('G'), modifiers: KeyModifiers::SHIFT, description: "Last Year", color: Color::Cyan },
    prev_year: KeyBinding { key: KeyCode::Char('g'), modifiers: KeyModifiers::NONE, description: "First Year (gg)", color: Color::Cyan },
    
    // Week navigation (vim-style)
    next_week: KeyBinding { key: KeyCode::Char('w'), modifiers: KeyModifiers::NONE, description: "Next Week", color: Color::Blue },
    prev_week: KeyBinding { key: KeyCode::Char('b'), modifiers: KeyModifiers::NONE, description: "Previous Week", color: Color::Blue },
    
    // Day navigation (vim-style)
    first_day_of_month: KeyBinding { key: KeyCode::Char('0'), modifiers: KeyModifiers::NONE, description: "First Day", color: Color::Blue },
    last_day_of_month: KeyBinding { key: KeyCode::Char('$'), modifiers: KeyModifiers::SHIFT, description: "Last Day", color: Color::Blue },
    
    // Go to today
    go_to_today: KeyBinding { key: KeyCode::Char('t'), modifiers: KeyModifiers::NONE, description: "Go to Today", color: Color::Magenta },
    
    // Task editing
    save_task: KeyBinding { key: KeyCode::Enter, modifiers: KeyModifiers::NONE, description: "Save", color: Color::Green },
    cancel_edit: KeyBinding { key: KeyCode::Esc, modifiers: KeyModifiers::NONE, description: "Cancel", color: Color::Red },
    switch_field: KeyBinding { key: KeyCode::Tab, modifiers: KeyModifiers::NONE, description: "Switch Field", color: Color::Green },
    backspace: KeyBinding { key: KeyCode::Backspace, modifiers: KeyModifiers::NONE, description: "Delete Char", color: Color::Gray },
    
    // App control
    quit: KeyBinding { key: KeyCode::Char('q'), modifiers: KeyModifiers::NONE, description: "Quit", color: Color::Red },
    quit_alt: KeyBinding { key: KeyCode::Esc, modifiers: KeyModifiers::NONE, description: "Quit", color: Color::Red },
    force_quit: KeyBinding { key: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, description: "Force Quit", color: Color::Red },
};

// ============================================================================
// EXAMPLES - To customize, edit the KEYBINDINGS constant above
// ============================================================================
//
// To change navigation to arrow keys, replace the movement bindings:
//   move_left: KeyBinding { key: KeyCode::Left, modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
//   move_down: KeyBinding { key: KeyCode::Down, modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
//   move_up: KeyBinding { key: KeyCode::Up, modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
//   move_right: KeyBinding { key: KeyCode::Right, modifiers: KeyModifiers::NONE, description: "Move", color: Color::Green },
//
// To change delete key from 'x' to 'd':
//   delete: KeyBinding { key: KeyCode::Char('d'), modifiers: KeyModifiers::NONE, description: "Delete", color: Color::Red },
//
// To add Ctrl modifier:
//   some_key: KeyBinding { key: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL, description: "Save", color: Color::Green },
//
// To add Shift modifier:
//   some_key: KeyBinding { key: KeyCode::Char('S'), modifiers: KeyModifiers::SHIFT, description: "Save All", color: Color::Green },

// Helper functions for UI
impl Config {
    pub fn get_normal_mode_help_spans(&self, can_undo: bool, can_redo: bool) -> Vec<ratatui::text::Span<'static>> {
        use ratatui::{text::Span, style::Style};
        
        let mut spans = Vec::new();
        
        // Movement keys (show as combined)
        spans.push(Span::styled("hjkl", Style::default().fg(Color::Green)));
        spans.push(Span::raw(": Move | "));
        
        // Task operations
        spans.push(Span::styled("i", Style::default().fg(self.insert_edit.color)));
        spans.push(Span::raw(": Insert/Edit | "));
        spans.push(Span::styled("x", Style::default().fg(self.delete.color)));
        spans.push(Span::raw(": Delete | "));
        spans.push(Span::styled("c", Style::default().fg(self.toggle_complete.color)));
        spans.push(Span::raw(": Toggle Complete | "));
        
        // Yank/Paste
        spans.push(Span::styled("y", Style::default().fg(self.yank.color)));
        spans.push(Span::raw(": Yank | "));
        spans.push(Span::styled("p", Style::default().fg(self.paste.color)));
        spans.push(Span::raw(": Paste | "));
        
        // Undo/Redo (only show if available)
        if can_undo {
            spans.push(Span::styled("u", Style::default().fg(self.undo.color)));
            spans.push(Span::raw(": Undo | "));
        }
        if can_redo {
            spans.push(Span::styled("Ctrl+r", Style::default().fg(self.redo.color)));
            spans.push(Span::raw(": Redo | "));
        }
        
        // Month/Year navigation (vim-style)
        spans.push(Span::styled("H/L", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(": Month | "));
        spans.push(Span::styled("gg/G", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(": Year | "));
        
        // Week navigation
        spans.push(Span::styled("w/b", Style::default().fg(self.next_week.color)));
        spans.push(Span::raw(": Week | "));
        
        // Day navigation
        spans.push(Span::styled("0/$", Style::default().fg(self.first_day_of_month.color)));
        spans.push(Span::raw(": Day | "));
        
        // Quit
        spans.push(Span::styled("q", Style::default().fg(self.quit.color)));
        spans.push(Span::raw(": Quit"));
        
        spans
    }
    
    pub fn get_edit_mode_help_spans(&self) -> Vec<ratatui::text::Span<'static>> {
        use ratatui::{text::Span, style::Style};
        
        vec![
            Span::styled("Tab", Style::default().fg(self.switch_field.color)),
            Span::raw(": Switch field | "),
            Span::styled("Enter", Style::default().fg(self.save_task.color)),
            Span::raw(": Save | "),
            Span::styled("Esc", Style::default().fg(self.cancel_edit.color)),
            Span::raw(": Cancel"),
        ]
    }
}
