use std::{error::Error, io};
use crossterm::{cursor::{Hide, Show}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    // adding audio 
    audio.add("explode", "target/explode.wav");
    audio.add("lose", "target/lose.wav");
    audio.add("move", "target/move.wav");
    audio.add("pew", "target/pew.wav");
    audio.add("startup", "target/startup.wav");
    audio.add("win", "target/win.wav");

    audio.play("win");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Finish cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}