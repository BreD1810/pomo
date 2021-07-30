use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use termion::{clear, cursor};

fn main() {
    ctrlc::set_handler(|| {
        println!("{}", cursor::Show);
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    let mut time = 1 * 60;
    let mut stdout = io::stdout();
    println!("{}", clear::All);
    println!("{}Welcome to the Pomodoro timer", cursor::Restore);

    while time != 0 {
        write!(
            stdout,
            "{}{}{}{}",
            clear::CurrentLine,
            cursor::Goto(1, 2),
            time,
            cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();
        time -= 1;
        sleep(Duration::from_secs(1));
    }
    println!("Done{}", cursor::Show);
}
