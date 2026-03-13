use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::PrintStyledContent("╬╬╬╬╬╬╬╬╬╬╬".magenta()))?;
    stdout
        .queue(cursor::MoveTo(0, 1))?
        .queue(style::PrintStyledContent("███████████".red()))?;
    stdout
        .queue(cursor::MoveTo(0, 2))?
        .queue(style::PrintStyledContent("▓▓▓▓▓▓▓▓▓▓▓".green()))?;
    stdout
        .queue(cursor::MoveTo(0, 3))?
        .queue(style::PrintStyledContent("▒▒▒▒▒▒▒▒▒▒▒".blue()))?;
    stdout
        .queue(cursor::MoveTo(0, 4))?
        .queue(style::PrintStyledContent("░░░░░░░░░░░".yellow()))?;
    stdout
        .queue(cursor::MoveTo(0, 5))?
        .queue(style::PrintStyledContent("##########".cyan()))?;
    stdout.flush()?;
    Ok(())
}
