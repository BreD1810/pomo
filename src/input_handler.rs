use std::io::stdin;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub enum InputCommand {
    Quit,
    PlayPause,
}

pub fn listen() -> Receiver<InputCommand> {
    let (sender, receiver) = mpsc::channel::<InputCommand>();
    thread::spawn(move || {
        let stdin = stdin().keys();
        for c in stdin {
            match c.unwrap() {
                Key::Char('q') | Key::Ctrl('c') => {
                    sender.send(InputCommand::Quit).unwrap();
                    break;
                },
                Key::Char('p') => {
                    sender.send(InputCommand::PlayPause).unwrap();
                },
                _ => (),
            }
        }
    });

    receiver
}
