use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};

mod cube;
mod position;

use crate::cube::Cube;

fn main() -> io::Result<()> {
    let width = 20;
    let height = 10;

    let mut cube = Cube::new();

    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    cube.apply_xz_rotation(20.0_f32.to_radians());

    for corner in cube.corners {
        let position2d = corner.transform_position_to_2d();
        let calculated_position = position2d.position_in_terminal_scale(width, height);
        stdout
            .queue(cursor::MoveTo(
                calculated_position.x as u16 + 2,
                calculated_position.y as u16 + 2,
            ))?
            .queue(style::PrintStyledContent("▆".magenta()))?;
    }

    stdout.flush()?;
    Ok(())
}
