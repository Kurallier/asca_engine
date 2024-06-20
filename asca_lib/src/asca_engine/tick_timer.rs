#[allow(dead_code)]
use asca_timer::Timer;

#[derive(Debug)]
pub struct TickTimer {
    pub timer: Timer,
    synced: bool,
    current_tick: u64,
}

impl Default for TickTimer {
    //Will create the timer with a 16.67TPS rate
    fn default() -> Self {
        let duration = std::time::Duration::from_millis(60);
        let timer = Timer::new(duration, std::time::Instant::now(), true);
        Self {
            timer,
            synced: false,
            current_tick: 0,
        }
    }
}

impl TickTimer {
    pub fn new(fps_ms: u32) -> Self {
        let duration = std::time::Duration::from_millis(fps_ms.into());
        let timer = Timer::new(duration, std::time::Instant::now(), true);
        Self {
            timer,
            ..Default::default()
        }
    }
    pub fn is_tick(&mut self) -> bool {
        if self.tick_sync().timer.timer_tick_nano().is_completed() {
            self.current_tick += 1;
            return true;
        } else {
            return false;
        }
    }

    fn tick_sync(&mut self) -> &mut Self {
        if self.synced {
            return self;
        };
        self.timer.reset_count();
        self.synced = true;
        self
    }
}
