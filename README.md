# Taskim

![TUI Demo](demo.gif)

Taskim is a terminal-based task manager built with Rust and [ratatui](https://github.com/ratatui-org/ratatui). It provides a Vim-inspired interface for managing tasks, navigating months, and customizing your workflow.

## Features

- **Monthly Calendar View:**  
  Visualize your tasks in a month grid, with navigation for days, weeks, months, and years.
- **Task Management:**  
  - Add, edit, and delete tasks for any date.
  - Tasks can have titles and optional content/comments.
  - Mark tasks as complete/incomplete.
  - Reorder tasks within a day.
- **Vim-style Keybindings:**  
  - Navigate with `h`, `j`, `k`, `l` or arrow keys.
  - Insert tasks above/below (`O`/`o`), delete (`dd`/`x`), yank/copy (`y`), paste (`p`/`P`), and undo/redo (`u/control-r`).
  - Command mode (`:`) for advanced actions (e.g., go to date, toggle wrap, show/hide keybinds).
- **Scramble Mode:**  
  Toggle (`s`) to obscure task names for privacy.
- **Customizable UI:**  
  - Colors and keybindings are configurable via `config.yml`.
  - Toggle keybind help bar and UI wrap mode.

## Getting Started

1. **Build and Run:**
   ```sh
   cargo run --release
   ```
2. **Configuration:**
   - Copy or edit config.yml in the project root to customize appearance and controls.
3. **Exit**
   - Quit with `q` or command mode `:wq`

## Motivation / Next Steps
The goal of this TUI was to replicate the features of the previous [task manager](https://github.com/RohanAdwankar/task-js) I have been using but be fully usable without a mouse using VIM motions.

At this point, the TUI is usable for me, but if there is some feature you would like to see, please let me know! (open an issue or PR)

That being said, here are some goals for the future:

- Full vim motions

  Right now the traversal is just what i ended up needing.

  Thereby it doesnt support '3j' in task traversal for example.

  It also doesnt support vim motions in the task edit view. 

  Since I don't need it, I will add it if someone asks for it.

- Migrate JS App Features
    
    There are several features missing currently like:
    * Search Bar
    * Activity Tracker
    * Alternative Task Views

### Command Mode (`:`) Reference

- `:q`, `:quit`, `:wq`, `:x` 
  Quit the application.

- `:help`, `:help <command>`
  Show help for command mode.

- `:seekeys`, `:set seekeys`  
  Show keybindings bar.

- `:nokeys`, `:set nokeys`  
  Hide keybindings bar.

- `:wrap`, `:set wrap`  
  Enable UI text wrapping.

- `:nowrap`, `:set nowrap`  
  Disable UI text wrapping.

- `:MM/DD/YYYY`, `:YYYY-MM-DD`, `:DD`, `:YYYY`
  Jump to a specific date in the calendar.

### Config Reference
- For the color customization options outside of the named colors, I use the Ratatui indexed colors. You can see how the numbers correspond to the colors [here](https://github.com/ratatui/ratatui/blob/main/examples/README.md#color-explorer).
