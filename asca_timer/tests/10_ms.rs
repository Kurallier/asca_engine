use asca_timer::Timer;

#[test]
fn one_shot_norm() -> Result<(), ()> {
    let mut timer = Timer::new(
        std::time::Duration::from_millis(250),
        std::time::Instant::now(),
        false,
    );

    loop {
        timer.timer_tick();
        if timer.timer_status() {
            break;
        }
    }

    return Ok(());
}

#[test]
fn one_shot_nano() -> Result<(), ()> {
    let mut timer = Timer::new(
        std::time::Duration::from_millis(250),
        std::time::Instant::now(),
        false,
    );

    loop {
        timer.timer_tick_nano();
        if timer.timer_status() {
            break;
        }
    }

    return Ok(());
}

#[test]
fn repeat_10() -> Result<(), ()> {
    let mut timer = Timer::new(
        std::time::Duration::from_millis(250),
        std::time::Instant::now(),
        true,
    );

    let passed: bool;

    loop {
        timer.timer_tick();

        if timer.timer_completions() == 10 {
            passed = true;
            break;
        } else if timer.timer_completions() >= 50 {
            passed = false;
            break;
        }
    }

    if passed == true {
        return Ok(());
    } else {
        return Err(());
    }
}

#[test]
fn repeat_10_nano() -> Result<(), ()> {
    let mut timer = Timer::new(
        std::time::Duration::from_millis(250),
        std::time::Instant::now(),
        true,
    );

    let passed: bool;

    loop {
        timer.timer_tick_nano();

        if timer.timer_completions() == 10 {
            passed = true;
            break;
        } else if timer.timer_completions() >= 50 {
            passed = false;
            break;
        }
    }

    if passed == true {
        return Ok(());
    } else {
        return Err(());
    }
}
