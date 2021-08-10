use pomo::*;

fn main() {
    let pomodoro = Pomodoro::new();
    pomodoro.run();

    shutdown(0);
}
