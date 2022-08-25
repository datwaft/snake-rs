mod game;
mod snake;

use game::Game;

use std::{io::stdout, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event},
    style, terminal, ExecutableCommand, Result,
};

fn main() -> Result<()> {
    stdout()
        .execute(terminal::EnterAlternateScreen)?
        .execute(style::ResetColor)?
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide)?
        .execute(cursor::MoveTo(0, 0))?;

    terminal::enable_raw_mode()?;

    let mut game = Game::new((20, 8));

    loop {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?
            .execute(style::Print(game.render()))?;

        if let Err(_) = game.tick() {
            break;
        }

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event::KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    event::KeyCode::Up => game.handle_up_event(),
                    event::KeyCode::Down => game.handle_down_event(),
                    event::KeyCode::Left => game.handle_left_event(),
                    event::KeyCode::Right => game.handle_right_event(),
                    event::KeyCode::Esc | event::KeyCode::Char('q') | event::KeyCode::Char('Q') => {
                        break;
                    }
                    event::KeyCode::Char('c') | event::KeyCode::Char('C') => {
                        if modifiers == event::KeyModifiers::CONTROL {
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    terminal::disable_raw_mode()?;

    stdout()
        .execute(style::ResetColor)?
        .execute(cursor::Show)?
        .execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}
