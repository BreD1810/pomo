use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use termion::{clear, cursor};

pub struct Pomodoro {
    stdout: io::Stdout,
    time: isize,
}

impl Pomodoro {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let time = 60;

        Pomodoro { stdout, time }
    }

    pub fn run(mut self) {
        while self.time != 0 {
            write!(
                self.stdout,
                "{}{}{}{}",
                clear::CurrentLine,
                cursor::Goto(1, 2),
                self.time,
                cursor::Hide
            )
            .unwrap();
            self.stdout.flush().unwrap();
            self.time -= 1;
            sleep(Duration::from_secs(1));
        }
    }
}

pub fn cleanup() {
    println!("{}{}", cursor::Goto(1, 3), cursor::Show);
}

pub fn print_skeleton() {
    println!("{}", clear::All);
    println!("{}Welcome to the Pomodoro timer", cursor::Restore);
    println!();
    println!("[q] Quit");
}

pub fn setup_ctrlc_handler() {
    ctrlc::set_handler(|| {
        cleanup();
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");
}
