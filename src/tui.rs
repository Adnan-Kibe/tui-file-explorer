use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::filesystem::App;

pub struct TUI;

impl TUI {
    pub fn app(&self, terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
        loop {
            terminal.draw(|frame| Self::render(frame, app))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Char('w') => app.move_selection(-1),
                    KeyCode::Char('s') => app.move_selection(1),
                    KeyCode::Enter => app.open_selected()?,
                    KeyCode::Backspace => app.go_back()?,
                    KeyCode::Down => app.scroll_preview_down(),
                    KeyCode::Up => app.scroll_preview_up(),
                    KeyCode::Char('o') => app.open_external()?,
                    _ => {}
                }
            }
        }
    }

    fn render(frame: &mut Frame, app: &mut App) {
        let areas = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let content_area = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(areas[2]);

        let header = Paragraph::new("My File Explorer")
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .title(" Application ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    ),
            );

        let items = app
            .files
            .iter()
            .map(|entry| {
                let marker = if entry.is_dir { "[DIR]" } else { "[FILE]" };
                ListItem::new(format!("{marker}{}", entry.name))
            })
            .collect::<Vec<ListItem>>();

        let files = List::new(items)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ")
            .block(Block::default().title("Files").borders(Borders::ALL));

        let path = Paragraph::new(app.current_dir.display().to_string()).block(
            Block::default()
                .title("Current Directory")
                .borders(Borders::ALL),
        );

        let preview = Paragraph::new(app.preview.as_str())
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .scroll((app.preview_scroll, 0));

        let footer = Paragraph::new(
            " W/S: Navigate  |  Enter: Open folder / Preview file  |  Backspace: Go back | Up/Down: Scroll preview  |  O: Open externally  |  Q/Esc: Quit"
        )
        .block(
            Block::default()
                .title("Commands")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(header, areas[0]);
        frame.render_widget(path, areas[1]);

        frame.render_stateful_widget(files, content_area[0], &mut app.list_state);
        frame.render_widget(preview, content_area[1]);

        frame.render_widget(footer, areas[3]);
    }
}
