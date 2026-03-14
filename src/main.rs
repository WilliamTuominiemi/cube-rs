use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event},
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::time::Duration;

mod cube;
mod position;

use crate::cube::Cube;

fn main() -> io::Result<()> {
    let width = 20;
    let height = 10;
    let mut quit = false;

    let mut cube = Cube::new();
    let mut stdout = io::stdout();

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    terminal::enable_raw_mode()?;
    while !quit {
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(_) = event::read()?
        {
            quit = true;
        }

        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        cube.apply_xz_rotation(5.0_f32.to_radians());

        for corner in &cube.corners {
            let position2d = corner.transform_position_to_2d();
            let calculated_position = position2d.position_in_terminal_scale(width, height);
            stdout
                .queue(cursor::MoveTo(
                    calculated_position.x as u16 + 2,
                    calculated_position.y as u16 + 2,
                ))?
                .queue(style::PrintStyledContent("▆".magenta()))?;
        }

        for line in &cube.get_lines() {
            let position2d = line.transform_position_to_2d();
            let calculated_position = position2d.position_in_terminal_scale(width, height);
            stdout
                .queue(cursor::MoveTo(
                    calculated_position.x as u16 + 2,
                    calculated_position.y as u16 + 2,
                ))?
                .queue(style::PrintStyledContent("•".magenta()))?;
        }

        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(10));
    }

    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}
