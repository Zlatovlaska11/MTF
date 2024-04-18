pub mod tui {

    use crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };
    use enigo::Key;
    use ratatui::{
        prelude::{CrosstermBackend, Stylize, Terminal},
        symbols::border,
        widgets::{
            block::{Position, Title},
            Block, Borders, Paragraph,
        },
    };
    use std::{
        io::{stdout, Result, Stdout},
        os::unix::thread,
        process::exit,
    };

    struct App {
        text: String,
        pos: i32,
    }
    use crate::typer::{self};
    //use crate::typer::typer;
    use ratatui::prelude::*;
    use std::thread::spawn;

    pub type Tui = Terminal<CrosstermBackend<Stdout>>;

    pub fn gui(text: String, should_type: bool) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        if should_type == true {
            let th = spawn(|| typer::typer::start_typing());
            let th_gui = spawn(|| gui("enabled".to_string(), false));
            
            th.join().unwrap();
            th_gui.join().unwrap();
        }

        loop {
            let title = Title::from(" Mount Blue Fucker ".bold());
            let instructions = Title::from(Line::from(vec![
                " StartTyping ".into(),
                "<e>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]));

            let block = Block::default()
                .title(title.alignment(Alignment::Center))
                .title(
                    instructions
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_set(border::THICK);

            terminal.draw(|frame| {
                let mut area = Rect::new(0, 0, frame.size().width, 10);
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .centered()
                        .blue()
                        .block(block),
                    area,
                );
                area = Rect::new(0, 10, (frame.size().width / 2) - 10, 10);
                frame.render_widget(
                    Paragraph::new(text.clone())
                        .centered()
                        .green()
                        .block(Block::new().borders(Borders::ALL)),
                    area,
                );
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    let _ = Terminal::clear(&mut terminal);
                    break;
                }
            }

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e') {
                    if should_type == true {
                        let _ = gui("enabled".to_string(), false);
                    } else {
                        let _ = gui("disabled".to_string(), true);
                    }
                }
            }
        }

        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        exit(69);
    }
}
