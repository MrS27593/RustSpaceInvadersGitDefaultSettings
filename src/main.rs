use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result < (), Box <dyn Error>> {
    let mut audio = Audio:: new();
    audio.add("explode" , "explode.wav");
    audio.add("lose" , "lose.wav");
    audio.add("move" , "move.wav");
    audio.add("pew" , "pew.wav");
    audio.add("startup" , "startup.wav");
    audio.add("win" , "win.wav");
    audio.add("startup");
    let mut stdout = io::stodout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?
    stdout.execute(Hide)?
    audio.wait();
    stdout.execute(Show)?
    stdout.execute(LeaveAlternateScreen)?
    terminal::disable_raw_mode;
    Ok()
    //GameLoop
    'gameloop: loop {
        //Input
        while event::poll(Duration::default())?{
            if lewt Event::Key(key_event) = event::read()?
        }
    }
}
