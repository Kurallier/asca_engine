use asca_timer::Timer;

fn main() {
    let mut timer = Timer::new(
        std::time::Duration::from_millis(1000),
        std::time::Instant::now(),
        false,
    );
    
    loop {
        timer.timer_tick();

        if timer.timer_status() == true {
            println!("works");
        }
    }
}
