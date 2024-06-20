use asca_timer::Timer;

fn main() {
    let mut timer = Timer::new(
        std::time::Duration::from_millis(33),
        std::time::Instant::now(),
        true,
    );

    loop {
        timer.timer_tick_nano();

        if timer.is_finished() {
            println!("Timer finished");
            println!("{}", timer.timer_completions());
            timer.timer_destroy();
            break;
        }
        if timer.is_completed() {
            println!("Timer Complete");
            println!("{}", timer.timer_completions());
            if timer.timer_completions() == 150 {
                timer.timer_destroy();
                break;
            }
        }
    }
}
