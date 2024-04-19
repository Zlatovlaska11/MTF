pub mod tui {

    use crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };

    use ratatui::{
        prelude::{CrosstermBackend, Stylize, Terminal},
        symbols::border,
        widgets::{
            block::{Position, Title},
            Block, Borders, Paragraph,
        },
    };
    use serde::{Deserialize, Serialize};
    use std::{
        fs,
        io::{stdout, Result},
        process::exit,
    };

    use crate::typer::{self, typer::Text};
    use ratatui::prelude::*;
    use std::thread::spawn;

    #[derive(Deserialize, Serialize)]
    pub struct Config {
        file_path: String,
    }

    static mut position: i32 = 0;

    pub fn gui(text: String, should_type: bool) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        if should_type == true {
            unsafe{

                let th = spawn(move || typer::typer::start_typing(&mut position));
                let _th_gui = spawn(|| gui("enabled".to_string(), false));

                th.join().unwrap();

                gui("disabled".to_string(), false).unwrap();

            }
            todo!("finsh the text display after finishing the write")
        }

        let pos = 0;
        loop {
            let mut tw = get_text();

            let ttw = get_colored_paragarph(&mut tw);

            let title = Title::from(" Mount Blue Fucker ".bold());
            let instructions = Title::from(Line::from(vec![
                " Start Typing ".into(),
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
                frame.render_widget(Paragraph::new(ttw).centered().blue().block(block), area);
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

    fn get_text() -> Text {
        let file_text = fs::read_to_string("config.json").unwrap();
        let conf: Config =
            serde_json::from_str(file_text.as_str()).expect("error reading the config file");

        typer::typer::get_file_input(&conf.file_path)
    }

    fn get_colored_paragarph(txt: &mut Text) -> Line {
        let mut text: Vec<Span> = Vec::new();

        let untyped = Style::default().fg(Color::Green);
        let _typed = Style::default().fg(Color::LightGreen);
        
        //add range implementation (0..pos ) == typed rest not typed
        for ch in &txt.text {
            match ch.col {
                typer::typer::Col::Typed => {
                    text.push(Span::styled(ch.key.to_string(), untyped));
                }
                typer::typer::Col::NotTyped => {
                    text.push(Span::styled(ch.key.to_string(), untyped));
                }
            }
        }

        text.into()
    }
}
