use std::error::Error;
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

    // test play
    audio.play("win");
    audio.wait();

    // test play
    audio.play("explode");
    audio.wait();

    Ok(())
}