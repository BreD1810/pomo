use pomo::*;

fn main() {
    setup_ctrlc_handler();

    let pomodoro = Pomodoro::new();
    print_header();
    pomodoro.run();

    shutdown(0);
}
