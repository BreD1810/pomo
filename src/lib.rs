use clap::{value_t, App, AppSettings, Arg};
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use termion::{clear, cursor};

pub struct Pomodoro {
    stdout: io::Stdout,

    /// The number of minutes for each Pomodoro.
    pomodoro_length: isize,

    /// The number of minutes for each short break.
    short: isize,

    /// The number of minutes for each long break.
    long: isize,
}

impl Pomodoro {
    pub fn new() -> Self {
        let matches = App::new("pomo")
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::DisableVersion)
            .arg(
                Arg::with_name("pomodoro")
                    .short("p")
                    .long("pomdoro")
                    .help("Number of minutes for each Pomodoro.")
                    .default_value("25"),
            )
            .arg(
                Arg::with_name("short_break")
                    .short("s")
                    .long("short")
                    .help("Number of minutes for a short break.")
                    .default_value("5"),
            )
            .arg(
                Arg::with_name("long_break")
                    .short("l")
                    .long("long")
                    .help("Number of minutes for a long break.")
                    .default_value("20"),
            )
            .get_matches();

        let pomodoro_length = value_t!(matches, "pomodoro", isize).unwrap();
        let long = value_t!(matches, "long_break", isize).unwrap();
        let short = value_t!(matches, "short_break", isize).unwrap();

        let stdout = io::stdout();

        Pomodoro {
            stdout,
            pomodoro_length,
            short,
            long,
        }
    }

    pub fn run(mut self) {
        let mut time = 60 * self.pomodoro_length;

        while time != 0 {
            write!(
                self.stdout,
                "{}{}{}{}",
                clear::CurrentLine,
                cursor::Goto(1, 2),
                time,
                cursor::Hide
            )
            .unwrap();
            self.stdout.flush().unwrap();
            time -= 1;
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
