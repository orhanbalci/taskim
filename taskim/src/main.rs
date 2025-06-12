mod config;
mod data;
mod month_view;
mod task;
mod task_edit;
mod undo;
mod utils;

use crate::config::KEYBINDINGS;
use crate::data::{load_data, save_data};
use crate::month_view::{render_month_view, MonthView, SelectionType};
use crate::task::TaskData;
use crate::task_edit::{render_task_edit_popup, TaskEditState};
use crate::undo::{Operation, UndoStack};
use crate::utils::days_in_month;

use chrono::{Datelike, Local, Timelike};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    style::{Style},
    text::{Line, Span},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

#[derive(Debug, Clone, PartialEq)]
enum AppMode {
    Normal,
    TaskEdit(TaskEditState),
    Command(CommandState),
}

#[derive(Debug, Clone, PartialEq)]
struct CommandState {
    input: String,
    cursor_position: usize,
    show_help: bool,
}

impl CommandState {
    fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            show_help: false,
        }
    }

    fn add_char(&mut self, ch: char) {
        self.input.insert(self.cursor_position, ch);
        self.cursor_position += 1;
    }

    fn remove_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    fn move_cursor_left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }
}

struct App {
    mode: AppMode,
    data: TaskData,
    month_view: MonthView,
    should_exit: bool,
    undo_stack: UndoStack,
    yanked_task: Option<crate::task::Task>, // Store yanked task for paste operation
    pending_key: Option<char>,              // For handling multi-key sequences like 'gg'
    pending_insert_order: Option<u32>,      // For tracking task insertion order
    scramble_mode: bool,                    // Toggle for scrambling task names with numbers
    config: crate::config::Config,          // <-- add config field
    show_keybinds: bool,                    // runtime toggle for keybind help
}

impl App {
    fn new() -> Self {
        let data = load_data();
        let current_date = Local::now().date_naive();
        let month_view = MonthView::new(current_date);
        let config = crate::config::Config::from_file_or_default("config.yml");
        let show_keybinds = config.show_keybinds;
        Self {
            mode: AppMode::Normal,
            data,
            month_view,
            should_exit: false,
            undo_stack: UndoStack::new(50), // Allow up to 50 undo operations
            yanked_task: None,
            pending_key: None,
            pending_insert_order: None,
            scramble_mode: false,
            config,
            show_keybinds,
        }
    }

    fn save(&self) -> Result<()> {
        save_data(&self.data).map_err(|e| color_eyre::eyre::eyre!(e))?;
        Ok(())
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match &self.mode {
            AppMode::Normal => self.handle_normal_mode_key(key)?,
            AppMode::Command(state) => {
                let mut new_state = state.clone();
                if self.handle_command_mode_key(key, &mut new_state)? {
                    // Command completed or cancelled
                    self.mode = AppMode::Normal;
                } else {
                    self.mode = AppMode::Command(new_state);
                }
            }
            AppMode::TaskEdit(state) => {
                let mut new_state = state.clone();
                if self.handle_task_edit_key(key, &mut new_state)? {
                    // Task edit completed
                    let mut task = new_state.to_task();
                    if new_state.is_new_task {
                        // Use pending insert order if set (for 'o' and 'O' commands)
                        if let Some(insert_order) = self.pending_insert_order.take() {
                            self.data.insert_task_at_order(task.clone(), insert_order);

                            // Select the new task by its order
                            let task_date = task.start.date_naive();
                            self.month_view.select_task_by_order(
                                task_date,
                                insert_order,
                                &self.data.events,
                            );
                        } else {
                            // Regular insertion (for 'i' command) - add to end
                            let task_date = task.start.date_naive();
                            task.order = self.data.max_order_for_date(task_date) + 1;
                            self.data.events.push(task.clone());
                        }

                        // Track task creation
                        self.undo_stack
                            .push(Operation::CreateTask { task: task.clone() });
                    } else {
                        // Track task edit
                        if let Some(existing) = self
                            .data
                            .events
                            .iter_mut()
                            .find(|t| Some(&t.id) == new_state.task_id.as_ref())
                        {
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
        if self.config.force_quit.matches(key.code, key.modifiers) {
            self.should_exit = true;
            return Ok(());
        }

        // Handle multi-key sequences first
        if let Some(pending) = self.pending_key {
            if pending == 'g'
                && key.code == KeyCode::Char('g')
                && key.modifiers == KeyModifiers::NONE
            {
                // Handle 'gg' - go to previous year
                self.month_view.prev_year();
                self.pending_key = None;
                return Ok(());
            } else if pending == 'd'
                && key.code == KeyCode::Char('d')
                && key.modifiers == KeyModifiers::NONE
            {
                // Handle 'dd' - cut the selected task (vim-style)
                if let Some(task_id) = self.month_view.get_selected_task_id() {
                    if let Some(task) = self.data.remove_task_and_reorder(&task_id) {
                        let task_date = task.start.date_naive();

                        // Store the cut task for pasting
                        self.yanked_task = Some(task.clone());

                        // Track deletion for undo functionality
                        self.undo_stack.push(Operation::DeleteTask {
                            task,
                            original_date: task_date,
                        });

                        // Check if there are any remaining tasks on the same date
                        let remaining_tasks = self.data.get_tasks_for_date(task_date);

                        if remaining_tasks.is_empty() {
                            // No more tasks on this day, select the day itself
                            self.month_view.selection = month_view::Selection {
                                selection_type: month_view::SelectionType::Day(task_date),
                                task_index_in_day: None,
                            };
                        } else {
                            // Select the first remaining task
                            self.month_view.selection = month_view::Selection {
                                selection_type: month_view::SelectionType::Task(
                                    remaining_tasks[0].id.clone(),
                                ),
                                task_index_in_day: Some(0),
                            };
                        }

                        self.save()?;
                    }
                }
                self.pending_key = None;
                return Ok(());
            }
            // If we have a pending key but don't match, clear it and continue with normal processing
            self.pending_key = None;
        }

        if self.config.quit.matches(key.code, key.modifiers)
            || self.config.quit_alt.matches(key.code, key.modifiers)
        {
            self.should_exit = true;
        } else if self.config.move_left.matches(key.code, key.modifiers) {
            self.month_view.move_left(&self.data.events);
        } else if self.config.move_down.matches(key.code, key.modifiers) {
            self.month_view.move_down(&self.data.events);
        } else if self.config.move_up.matches(key.code, key.modifiers) {
            self.month_view.move_up(&self.data.events);
        } else if self.config.move_right.matches(key.code, key.modifiers) {
            self.month_view.move_right(&self.data.events);
        } else if self.config.insert_edit.matches(key.code, key.modifiers) {
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
            }
        } else if self.config.insert_below.matches(key.code, key.modifiers) {
            // Insert task below current position (vim-style: o)
            let selected_date = self.month_view.get_selected_date(&self.data.events);
            let edit_state = TaskEditState::new_task(selected_date);

            // Store the insertion order for when the task is created
            let insert_order = if let Some(current_order) =
                self.month_view.get_current_task_order(&self.data.events)
            {
                current_order + 1
            } else {
                self.data.max_order_for_date(selected_date) + 1
            };

            // We'll need to track this order for when the task gets created
            // For now, set up the task edit state
            self.pending_insert_order = Some(insert_order);
            self.mode = AppMode::TaskEdit(edit_state);
        } else if self.config.insert_above.matches(key.code, key.modifiers) {
            // Insert task above current position (vim-style: O)
            let selected_date = self.month_view.get_selected_date(&self.data.events);
            let edit_state = TaskEditState::new_task(selected_date);

            // Store the insertion order for when the task is created
            let insert_order = if let Some(current_order) =
                self.month_view.get_current_task_order(&self.data.events)
            {
                current_order
            } else {
                0
            };

            // We'll need to track this order for when the task gets created
            self.pending_insert_order = Some(insert_order);
            self.mode = AppMode::TaskEdit(edit_state);
        } else if self.config.delete_line.matches(key.code, key.modifiers) {
            // Handle first 'd' for 'dd' sequence
            self.pending_key = Some('d');
        } else if self.config.delete.matches(key.code, key.modifiers) {
            // Delete/cut the selected task (vim-style 'x') - same as 'dd'
            if let Some(task_id) = self.month_view.get_selected_task_id() {
                if let Some(deleted_task) = self.data.remove_task_and_reorder(&task_id) {
                    let task_date = deleted_task.start.date_naive();

                    // Store the cut task for pasting (copy functionality)
                    self.yanked_task = Some(deleted_task.clone());

                    // Track deletion for undo functionality
                    self.undo_stack.push(Operation::DeleteTask {
                        task: deleted_task,
                        original_date: task_date,
                    });

                    // Check if there are any remaining tasks on the same date
                    let remaining_tasks = self.data.get_tasks_for_date(task_date);

                    if remaining_tasks.is_empty() {
                        // No more tasks on this day, select the day itself
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Day(task_date),
                            task_index_in_day: None,
                        };
                    } else {
                        // Select the first remaining task (ordered)
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Task(
                                remaining_tasks[0].id.clone(),
                            ),
                            task_index_in_day: Some(0),
                        };
                    }

                    self.save()?;
                }
            }
        } else if self.config.undo.matches(key.code, key.modifiers) {
            // Undo last operation
            if let Some(operation) = self.undo_stack.undo() {
                match operation {
                    Operation::DeleteTask {
                        task,
                        original_date: _,
                    } => {
                        // Restore deleted task
                        self.data.events.push(task.clone());

                        // Select the restored task
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Task(task.id),
                            task_index_in_day: Some(0),
                        };
                    }
                    Operation::EditTask {
                        task_id,
                        old_task,
                        new_task: _,
                    } => {
                        // Revert task edit
                        if let Some(existing) =
                            self.data.events.iter_mut().find(|t| t.id == task_id)
                        {
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
                    Operation::YankPaste {
                        task_id,
                        old_date,
                        new_date: _,
                    } => {
                        // TODO: Implement when yank/paste is added
                        // For now, we'll revert the task to its old date
                        if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                            let duration = task.end - task.start;
                            let old_datetime = old_date
                                .and_hms_opt(
                                    task.start.time().hour(),
                                    task.start.time().minute(),
                                    task.start.time().second(),
                                )
                                .unwrap()
                                .and_utc();
                            task.start = old_datetime;
                            task.end = old_datetime + duration;
                        }
                    }
                }
                self.save()?;
            }
        } else if self.config.redo.matches(key.code, key.modifiers) {
            // Redo last undone operation
            if let Some(operation) = self.undo_stack.redo() {
                match operation {
                    Operation::DeleteTask {
                        task,
                        original_date: _,
                    } => {
                        // Re-delete the task
                        self.data.events.retain(|t| t.id != task.id);

                        // Select the day where the task was
                        let task_date = task.start.date_naive();
                        self.month_view.selection = month_view::Selection {
                            selection_type: month_view::SelectionType::Day(task_date),
                            task_index_in_day: None,
                        };
                    }
                    Operation::EditTask {
                        task_id,
                        old_task: _,
                        new_task,
                    } => {
                        // Re-apply task edit
                        if let Some(existing) =
                            self.data.events.iter_mut().find(|t| t.id == task_id)
                        {
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
                    Operation::YankPaste {
                        task_id,
                        old_date: _,
                        new_date,
                    } => {
                        // TODO: Implement when yank/paste is added
                        if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                            let duration = task.end - task.start;
                            let new_datetime = new_date
                                .and_hms_opt(
                                    task.start.time().hour(),
                                    task.start.time().minute(),
                                    task.start.time().second(),
                                )
                                .unwrap()
                                .and_utc();
                            task.start = new_datetime;
                            task.end = new_datetime + duration;
                        }
                    }
                }
                self.save()?;
            }
        } else if self.config.toggle_complete.matches(key.code, key.modifiers) {
            // Toggle task completion
            if let Some(task_id) = self.month_view.get_selected_task_id() {
                if let Some(task) = self.data.events.iter_mut().find(|t| t.id == task_id) {
                    task.completed = !task.completed;
                    self.save()?;
                }
            }
        } else if self.config.yank.matches(key.code, key.modifiers) {
            // Yank (copy) task
            if let Some(task_id) = self.month_view.get_selected_task_id() {
                if let Some(task) = self.data.events.iter().find(|t| t.id == task_id) {
                    self.yanked_task = Some(task.clone());
                }
            }
        } else if self.config.paste.matches(key.code, key.modifiers) {
            // Paste task below current position
            if let Some(yanked_task) = &self.yanked_task {
                let selected_date = self.month_view.get_selected_date(&self.data.events);
                let mut new_task = yanked_task.clone();

                // Generate new ID for the pasted task
                new_task.id = format!(
                    "task_{}",
                    chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
                );

                // Set new start/end times for the selected date
                let duration = new_task.end - new_task.start;
                let new_start = selected_date
                    .and_hms_opt(
                        new_task.start.time().hour(),
                        new_task.start.time().minute(),
                        new_task.start.time().second(),
                    )
                    .unwrap()
                    .and_utc();
                new_task.start = new_start;
                new_task.end = new_start + duration;

                // Insert task with proper ordering
                let insert_order = if let Some(current_order) =
                    self.month_view.get_current_task_order(&self.data.events)
                {
                    current_order + 1
                } else {
                    self.data.max_order_for_date(selected_date) + 1
                };

                self.data
                    .insert_task_at_order(new_task.clone(), insert_order);

                // Track the paste operation for undo
                self.undo_stack.push(Operation::CreateTask {
                    task: new_task.clone(),
                });

                // Select the new task
                self.month_view.select_task_by_order(
                    selected_date,
                    insert_order,
                    &self.data.events,
                );
                self.save()?;
            }
        } else if self.config.paste_above.matches(key.code, key.modifiers) {
            // Paste task above current position
            if let Some(yanked_task) = &self.yanked_task {
                let selected_date = self.month_view.get_selected_date(&self.data.events);
                let mut new_task = yanked_task.clone();

                // Generate new ID for the pasted task
                new_task.id = format!(
                    "task_{}",
                    chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
                );

                // Set new start/end times for the selected date
                let duration = new_task.end - new_task.start;
                let new_start = selected_date
                    .and_hms_opt(
                        new_task.start.time().hour(),
                        new_task.start.time().minute(),
                        new_task.start.time().second(),
                    )
                    .unwrap()
                    .and_utc();
                new_task.start = new_start;
                new_task.end = new_start + duration;

                // Insert task with proper ordering (above current)
                let insert_order = if let Some(current_order) =
                    self.month_view.get_current_task_order(&self.data.events)
                {
                    current_order
                } else {
                    0
                };

                self.data
                    .insert_task_at_order(new_task.clone(), insert_order);

                // Track the paste operation for undo
                self.undo_stack.push(Operation::CreateTask {
                    task: new_task.clone(),
                });

                // Select the new task
                self.month_view.select_task_by_order(
                    selected_date,
                    insert_order,
                    &self.data.events,
                );
                self.save()?;
            }
        } else if self.config.next_month.matches(key.code, key.modifiers) {
            // Next month (vim-style: L) - preserve day
            self.month_view.next_month_preserve_day();
        } else if self.config.prev_month.matches(key.code, key.modifiers) {
            // Previous month (vim-style: H) - preserve day
            self.month_view.prev_month_preserve_day();
        } else if self.config.next_year.matches(key.code, key.modifiers) {
            // Next year (vim-style: G)
            self.month_view.next_year();
        } else if self.config.prev_year.matches(key.code, key.modifiers) {
            // Handle first 'g' for 'gg' sequence
            self.pending_key = Some('g');
        } else if self.config.go_to_today.matches(key.code, key.modifiers) {
            // Go to today (vim-style: t)
            self.month_view.go_to_today();
        } else if self.config.next_week.matches(key.code, key.modifiers) {
            // Next week (vim-style: w)
            self.month_view.next_week(&self.data.events);
        } else if self.config.prev_week.matches(key.code, key.modifiers) {
            // Previous week (vim-style: b)
            self.month_view.prev_week(&self.data.events);
        } else if self
            .config
            .first_day_of_month
            .matches(key.code, key.modifiers)
        {
            // First day of month (vim-style: 0)
            self.month_view.first_day_of_month();
        } else if self
            .config
            .last_day_of_month
            .matches(key.code, key.modifiers)
            || (key.code == KeyCode::Char('$') && key.modifiers == KeyModifiers::NONE)
        {
            // Last day of month (vim-style: $) - handle both shift+4 and direct $
            self.month_view.last_day_of_month();
        } else if key.code == KeyCode::Char(':') && key.modifiers == KeyModifiers::NONE {
            // Enter command mode (vim-style: :)
            self.mode = AppMode::Command(CommandState::new());
        } else if key.code == KeyCode::Char('s') && key.modifiers == KeyModifiers::NONE {
            // Toggle scramble mode
            self.scramble_mode = !self.scramble_mode;
        }
        Ok(())
    }

    fn handle_task_edit_key(
        &mut self,
        key: crossterm::event::KeyEvent,
        state: &mut TaskEditState,
    ) -> Result<bool> {
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

    fn handle_command_mode_key(
        &mut self,
        key: crossterm::event::KeyEvent,
        state: &mut CommandState,
    ) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                // Cancel command mode
                return Ok(true);
            }
            KeyCode::Enter => {
                // Execute command
                let command = state.input.trim();

                if command == "help" {
                    // Toggle help display
                    state.show_help = !state.show_help;
                    state.input.clear();
                    state.cursor_position = 0;
                    return Ok(false); // Stay in command mode to show help
                } else if !command.is_empty() {
                    if let Err(e) = self.execute_command(&state.input) {
                        // For now, just return to normal mode on any error
                        // TODO: Add error display
                        eprintln!("Command error: {}", e);
                    }
                    return Ok(true);
                } else {
                    // Empty command, just exit
                    return Ok(true);
                }
            }
            KeyCode::Backspace => {
                state.remove_char();
                // Hide help when user starts typing
                state.show_help = false;
            }
            KeyCode::Left => {
                state.move_cursor_left();
            }
            KeyCode::Right => {
                state.move_cursor_right();
            }
            KeyCode::Char(ch) => {
                state.add_char(ch);
                // Hide help when user starts typing
                state.show_help = false;
            }
            _ => {}
        }
        Ok(false)
    }

    fn execute_command(&mut self, command: &str) -> Result<()> {
        let trimmed = command.trim();
        if trimmed == ":set seekeys" || trimmed == "set seekeys" || trimmed == "seekeys" {
            self.show_keybinds = true;
            return Ok(());
        } else if trimmed == ":set nokeys" || trimmed == "set nokeys" || trimmed == "nokeys" {
            self.show_keybinds = false;
            return Ok(());
        }

        if trimmed.is_empty() {
            return Ok(());
        }

        // Handle quit commands (vim-style)
        match trimmed {
            "q" | "quit" => {
                self.should_exit = true;
                return Ok(());
            }
            "q!" | "quit!" => {
                // Force quit without saving
                self.should_exit = true;
                return Ok(());
            }
            "wq" | "x" => {
                // Write and quit (save and exit)
                self.save()?;
                self.should_exit = true;
                return Ok(());
            }
            _ => {}
        }

        // Handle help command
        if trimmed == "help" {
            // Show help in footer by temporarily switching modes - we'll handle this differently
            // For now, just return Ok since help is shown in the UI
            return Ok(());
        }

        // Handle wrap commands
        match trimmed {
            "set wrap" | "wrap" => {
                self.month_view.set_wrap(true);
                return Ok(());
            }
            "set nowrap" | "nowrap" => {
                self.month_view.set_wrap(false);
                return Ok(());
            }
            _ => {}
        }

        // Try to parse as a date in various formats
        if let Some(date) = self.parse_date_command(trimmed) {
            // Navigate to the specified date using the existing methods
            if date.month() != self.month_view.current_date.month()
                || date.year() != self.month_view.current_date.year()
            {
                self.month_view.current_date = date.with_day(1).unwrap();
                self.month_view.weeks =
                    MonthView::build_weeks_for_date(self.month_view.current_date);
            }

            self.month_view.selection = month_view::Selection {
                selection_type: month_view::SelectionType::Day(date),
                task_index_in_day: None,
            };

            return Ok(());
        }

        Err(color_eyre::eyre::eyre!(
            "Unknown command: {}. Type ':help' for available commands.",
            trimmed
        ))
    }

    fn parse_date_command(&self, input: &str) -> Option<chrono::NaiveDate> {
        use chrono::NaiveDate;

        // Try parsing as YYYY (year only)
        if let Ok(year) = input.parse::<i32>() {
            if year >= 1900 && year <= 2050 {
                let current_month = self.month_view.current_date.month();
                let current_day = self.month_view.get_selected_date(&self.data.events).day();

                // Calculate days in the target month for the specified year
                let days_in_month = days_in_month(year, current_month);

                let safe_day = std::cmp::min(current_day, days_in_month);
                return NaiveDate::from_ymd_opt(year, current_month, safe_day);
            }
        }

        // Try parsing as MM/DD/YYYY (simple manual parsing)
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() == 3 {
            if let (Ok(month), Ok(day), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<i32>(),
            ) {
                return NaiveDate::from_ymd_opt(year, month, day);
            }
        }

        // Try parsing as DD (day only)
        if let Ok(day) = input.parse::<u32>() {
            if day >= 1 && day <= 31 {
                let current_year = self.month_view.current_date.year();
                let current_month = self.month_view.current_date.month();

                // Check if the day is valid for the current month
                let days_in_month = days_in_month(current_year, current_month);

                if day <= days_in_month {
                    return NaiveDate::from_ymd_opt(current_year, current_month, day);
                }
            }
        }

        None
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

        // Create main layout - adjust footer size based on command mode
        let footer_height = match &self.mode {
            AppMode::Command(state) if state.show_help => 7, // More space for help (added wrap commands)
            _ => 2,                                          // Normal footer size
        };

        let layout = Layout::vertical([
            Constraint::Min(0),                // Main content
            Constraint::Length(footer_height), // Footer
        ])
        .split(area);

        // Render main content
        render_month_view(
            frame,
            layout[0],
            &self.month_view,
            &self.data.events,
            self.scramble_mode,
            &self.config,
        );

        // Render footer
        self.render_footer(frame, layout[1]);

        // Render mode-specific overlays
        match &self.mode {
            AppMode::TaskEdit(state) => {
                render_task_edit_popup(frame, area, state, &self.config);
            }
            AppMode::Command(_) => {
                // Command mode is handled in the footer
            }
            AppMode::Normal => {}
        }
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        match &self.mode {
            AppMode::Command(state) => {
                if state.show_help {
                    let help_lines = vec![
                        Line::from(vec![
                            Span::styled("Date Navigation Commands:", Style::default().fg(self.config.ui_colors.selected_task_fg)),
                        ]),
                        Line::from(vec![
                            Span::styled("YYYY", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Go to year (e.g., 2024) | "),
                            Span::styled("DD", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Go to day in current month (e.g., 15)"),
                        ]),
                        Line::from(vec![
                            Span::styled("MM/DD/YYYY", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Go to specific date (e.g., 06/15/2024)"),
                        ]),
                        Line::from(vec![
                            Span::styled("Quit Commands:", Style::default().fg(self.config.ui_colors.completed_task_fg)),
                            Span::raw(" "),
                            Span::styled(":q", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Quit | "),
                            Span::styled(":wq", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Save & quit | "),
                            Span::styled(":q!", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Force quit"),
                        ]),
                        Line::from(vec![
                            Span::styled("Display Commands:", Style::default().fg(self.config.ui_colors.completed_task_fg)),
                            Span::raw(" "),
                            Span::styled(":set wrap", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Enable text wrapping | "),
                            Span::styled(":set nowrap", Style::default().fg(self.config.ui_colors.selected_task_bg)),
                            Span::raw(" - Disable text wrapping"),
                        ]),
                        Line::from(vec![
                            Span::styled(":help", Style::default().fg(self.config.ui_colors.selected_completed_task_fg)),
                            Span::raw(" - Toggle this help | "),
                            Span::styled("Esc", Style::default().fg(self.config.ui_colors.selected_completed_task_bg)),
                            Span::raw(" - Exit command mode"),
                        ]),
                    ];
                    let help_paragraph = Paragraph::new(help_lines)
                        .style(Style::default().fg(self.config.ui_colors.default_fg));
                    frame.render_widget(help_paragraph, area);
                } else {
                    let command_line = format!(":{}", state.input);
                    let command_paragraph = Paragraph::new(command_line.as_str())
                        .style(Style::default().fg(self.config.ui_colors.default_fg));
                    frame.render_widget(command_paragraph, area);
                    frame.set_cursor_position(Position::new(
                        area.x + 1 + state.cursor_position as u16,
                        area.y
                    ));
                }
            }
            AppMode::Normal => {
                if self.show_keybinds {
                    let spans = self.config.get_normal_mode_help_spans(
                        self.undo_stack.can_undo(),
                        self.undo_stack.can_redo()
                    );
                    let help_text = vec![Line::from(spans)];
                    let footer = Paragraph::new(help_text)
                        .style(Style::default().fg(self.config.ui_colors.default_fg));
                    frame.render_widget(footer, area);
                } else {
                    let footer = Paragraph::new("").style(Style::default().fg(self.config.ui_colors.default_fg));
                    frame.render_widget(footer, area);
                }
            }
            AppMode::TaskEdit(_) => {
                let spans = self.config.get_edit_mode_help_spans();
                let help_text = vec![Line::from(spans)];
                let footer = Paragraph::new(help_text)
                    .style(Style::default().fg(self.config.ui_colors.default_fg));
                frame.render_widget(footer, area);
            }
        }
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
