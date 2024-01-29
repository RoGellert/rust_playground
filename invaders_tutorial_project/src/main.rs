use std::{error::Error, io::{self}, sync::mpsc, thread, time::Duration};
use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use invaders_tutorial_project::{frame::{self, new_frame}, render};
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

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    audio.play("startup");

    'gameloop: loop {
        // per-frame init
        let curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Finish cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}