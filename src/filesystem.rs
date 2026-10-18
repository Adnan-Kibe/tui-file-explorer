use std::{env, fs, io, path::PathBuf};

use ratatui::widgets::ListState;

pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
}

pub struct App {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub list_state: ListState,
    pub preview: String,
    pub preview_scroll: u16,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let current_dir = env::current_dir()?;

        let mut app = Self {
            current_dir: PathBuf::new(),
            files: Vec::new(),
            list_state: ListState::default(),
            preview: String::from("Select a file to preview"),
            preview_scroll: 0
        };

        app.load_directory(current_dir)?;

        Ok(app)
    }

    fn load_directory(&mut self, path: PathBuf) -> io::Result<()> {
        let mut files = Vec::new();

        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = entry.file_type()?.is_dir();

            files.push(FileEntry { name, is_dir });
        }

        files.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        self.current_dir = path;
        self.files = files;
        self.list_state
            .select(if self.files.is_empty() { None } else { Some(0) });

        Ok(())
    }

    pub fn open_selected(&mut self) -> io::Result<()> {
        let Some(index) = self.list_state.selected() else {
            return Ok(());
        };

        let Some(entry) = self.files.get(index) else {
            return Ok(());
        };

        let path = self.current_dir.join(&entry.name);

        if entry.is_dir {
            self.load_directory(path)?;
            self.preview = String::from("Select a file to preview");
        } else {
            self.preview = fs::read_to_string(path)
                .unwrap_or_else(|_| String::from("Unable to preview this file"));
        }

        Ok(())
    }
    pub fn go_back(&mut self) -> io::Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.load_directory(parent.to_path_buf())?;
        }

        Ok(())
    }

    pub fn move_selection(&mut self, direction: i32) {
        let len = self.files.len();

        if len == 0 {
            return;
        }

        let selected = self.list_state.selected().unwrap_or(0);

        let next = if direction < 0 {
            selected.saturating_sub(1)
        } else {
            (selected + 1).min(len - 1)
        };

        self.list_state.select(Some(next));
    }

    pub fn scroll_preview_down(&mut self){
        self.preview_scroll = self.preview_scroll.saturating_add(1)
    }

    pub fn scroll_preview_up(&mut self){
        self.preview_scroll = self.preview_scroll.saturating_sub(1)
    }

    pub fn open_external(&self) -> io::Result<()> {
        let Some(index) = self.list_state.selected() else {
            return Ok(());
        };

        let Some(entry) = self.files.get(index) else {
            return Ok(());
        };

        if entry.is_dir {
            return Ok(());
        }

        let path = self.current_dir.join(&entry.name);

        open::that(path).map_err(|err| io::Error::other(err.to_string()))?;

        Ok(())
    }
}
