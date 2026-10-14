# Rust Terminal File Explorer

A simple terminal-based file explorer built with **Rust** and **Ratatui**. This mini project explores how to build interactive terminal user interfaces (TUIs), handle keyboard events, navigate the filesystem, and manage application state in Rust.

## Features

- **Directory listing:** Reads and displays files and folders from the current directory.
- **Directory navigation:** Open folders with `Enter` and return to the parent directory with `Backspace`.
- **Keyboard navigation:** Use the `Up` and `Down` arrow keys to move through entries.
- **File and folder identification:** Displays `[DIR]` and `[FILE]` markers to distinguish directories from regular files.
- **Current directory display:** Shows the path of the directory currently being explored.
- **Text file preview:** Press `Enter` on a file to display its contents in the preview panel.
- **Selection highlighting:** Highlights the currently selected entry.
- **Graceful exit:** Press `q` or `Esc` to exit the application.

## Technologies Used

- **Rust** — application logic and filesystem operations.
- **Ratatui** — terminal user interface and widgets.
- **Crossterm** — keyboard input and terminal event handling.

## Prerequisites

Before running the application, install:

1. [Rust and Cargo](https://www.rust-lang.org/tools/install).
2. A terminal that supports the keyboard input and rendering features used by Crossterm and Ratatui.

Verify your installation:

```bash
rustc --version
cargo --version
```

## Installation

### 1. Clone the repository

If the project is hosted on GitHub, clone it:

```bash
git clone <repository-url>
cd <project-directory>
```

Replace the placeholders with your repository URL and project directory.

Alternatively, navigate directly to your existing project folder.

### 2. Check the dependencies

Ensure your `Cargo.toml` includes the required crates. For example:

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
```

These are example versions. Use the versions already in your `Cargo.toml` if your project is configured differently.

### 3. Run the application

From the directory containing `Cargo.toml`, execute:

```bash
cargo run
```

Cargo will compile the project and launch the terminal file explorer.

To build an executable without immediately running it:

```bash
cargo build --release
```

## Keyboard Shortcuts

| Key | Action |
|---|---|
| `↑` | Select the previous entry |
| `↓` | Select the next entry |
| `Enter` | Open a directory or preview a file |
| `Backspace` | Navigate to the parent directory |
| `q` | Quit the application |
| `Esc` | Quit the application |

## How It Works

### Application state

The application maintains its state in an `App` struct, including:

- `current_dir`: the path of the directory being explored.
- `files`: a collection of file entries and their types.
- `list_state`: the selected entry in the Ratatui list.
- `preview`: the text currently displayed in the preview panel.

### Filesystem operations

Rust's standard library handles filesystem access through `std::fs`:

- `fs::read_dir()` retrieves entries within a directory.
- `entry.file_type()?.is_dir()` identifies directories.
- `PathBuf` stores and combines filesystem paths.
- `fs::read_to_string()` reads text files for preview.

### Terminal interface

Ratatui renders the interface using widgets such as `List`, `Paragraph`, and `Block`. Layouts divide the terminal into areas for the header, current directory, file list, preview panel, and footer.

Crossterm supplies keyboard events. The application handles these events in a loop and updates its state before rendering the interface again.

## Limitations

- File previews are intended for text files; binary files may not display correctly.
- Large files are read entirely into memory when previewed.
- Filesystem access can fail because of permissions or other operating-system restrictions.
- The application currently previews files when `Enter` is pressed rather than updating the preview automatically as the selection changes.
- File operations such as editing, deleting, renaming, and copying are not implemented.

## Future Improvements

Potential additions include:

- Automatic preview updates when moving between files.
- File size, modification date, and file permissions.
- Improved handling of unreadable files and filesystem errors.
- File-type-specific icons and syntax highlighting.
- Search and filtering.
- File editing, copying, moving, renaming, and deletion.
- Support for sending files between storage nodes over QUIC.


