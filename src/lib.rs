use clap::{value_t, App, AppSettings, Arg};
use figlet_rs::FIGfont;
use input_handler::InputCommand;
use std::io;
use std::io::Stdout;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};

mod input_handler;

pub struct Pomodoro {
    /// Is the timer paused?
    paused: bool,

    /// Number of Pomodoros completed
    counter: i8,

    /// The number of minutes for each Pomodoro.
    pomodoro_length: isize,

    /// The number of minutes for each short break.
    short: isize,

    /// The number of minutes for each long break.
    long: isize,
}

enum BreakType {
    Short,
    Long,
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

        Pomodoro {
            paused: true,
            counter: 1,
            pomodoro_length,
            short,
            long,
        }
    }

    pub fn run(mut self) {
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let input_receiver = input_handler::listen();

        let mut break_type: Option<BreakType>;

        self.print_welcome(&mut stdout);
        while self.paused {
            match input_receiver.try_recv() {
                Ok(input_handler::InputCommand::Quit) => shutdown(0),
                Ok(input_handler::InputCommand::PlayPause) => {
                    self.paused = !self.paused;
                }
                _ => (),
            }
        }

        loop {
            break_type = None;
            self.print_header(&mut stdout, &break_type);
            self.countdown(&break_type, &mut stdout, &input_receiver);
            self.counter += 1;

            // Break
            if self.counter == 5 {
                break_type = Some(BreakType::Long);
                self.counter = 1;
            } else {
                break_type = Some(BreakType::Short);
            }

            self.print_header(&mut stdout, &break_type);
            self.countdown(&break_type, &mut stdout, &input_receiver);
        }
    }

    fn print_welcome(&self, stdout: &mut RawTerminal<Stdout>) {
        let font = FIGfont::standand().unwrap();
        let title = font.convert("POMO").unwrap().to_string().replace("\n", "\n\r");

        write!(stdout, "{}{}{}{}", clear::All, cursor::Goto(1, 1), title, cursor::Hide).unwrap();
        println!("[q] Quit\t[p] Start");
    }

    fn print_header(&self, stdout: &mut RawTerminal<Stdout>, break_type: &Option<BreakType>) {
        writeln!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        match break_type {
            Some(BreakType::Long) => writeln!(stdout, "You have a long break!").unwrap(),
            Some(BreakType::Short) => writeln!(stdout, "You have a short break!").unwrap(),
            None => writeln!(stdout, "You are on Pomodoro {}", self.counter).unwrap(),
        }
    }

    fn countdown(
        &mut self,
        break_type: &Option<BreakType>,
        stdout: &mut RawTerminal<Stdout>,
        input_receiver: &Receiver<InputCommand>,
    ) {
        let font = FIGfont::standand().unwrap();

        let mut time = match break_type {
            Some(BreakType::Long) => 60 * self.long,
            Some(BreakType::Short) => 60 * self.short,
            None => 60 * self.pomodoro_length,
        };

        while time != 0 {
            match input_receiver.try_recv() {
                Ok(input_handler::InputCommand::Quit) => shutdown(0),
                Ok(input_handler::InputCommand::PlayPause) => {
                    self.paused = !self.paused;
                }
                _ => (),
            }

            if !self.paused {
                let minutes = time / 60;
                let seconds = time % 60;
                let time_string = font
                    .convert(&format!("{}:{:02}", minutes, seconds))
                    .unwrap()
                    .to_string()
                    .replace("\n", "\n\r");

                write!(stdout, "{}{}", cursor::Goto(1, 2), clear::AfterCursor).unwrap();

                write!(
                    stdout,
                    "{}{}{}",
                    clear::CurrentLine,
                    cursor::Goto(1, 2),
                    time_string,
                )
                .unwrap();

                println!("[q] Quit\t[p] Play/Pause");

                time -= 1;
                spin_sleep::sleep(Duration::from_secs(1));
            } else {
                write!(stdout, "\rPAUSED",).unwrap();

                stdout.flush().unwrap();
            }
        }
    }
}

pub fn shutdown(exit_code: i32) {
    println!("{}{}{}", clear::All, cursor::Restore, cursor::Show);
    std::process::exit(exit_code);
}
