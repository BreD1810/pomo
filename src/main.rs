use pomo::*;

fn main() {

    let pomodoro = Pomodoro::new();
    print_header();
    pomodoro.run();

    shutdown(0);
}
