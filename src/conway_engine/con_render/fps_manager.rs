use asca_timer::Timer;

#[derive(Debug)]
pub struct FrameRateManager {
    pub frame_timer: Timer,
    synced: bool,
}

impl Default for FrameRateManager {
    //Will create the fps_manager with a 200Hz rate
    fn default() -> Self {
        let duration = std::time::Duration::from_millis(5);
        let timer = Timer::new(duration, std::time::Instant::now(), true);
        Self {
            frame_timer: timer,
            synced: false,
        }
    }
}

impl FrameRateManager {
    pub fn new(fps_ms: u32) -> Self {
        let duration = std::time::Duration::from_millis(fps_ms.into());
        let timer = Timer::new(duration, std::time::Instant::now(), true);
        Self {
            frame_timer: timer,
            synced: false,
        }
    }
    pub fn should_draw(&mut self) -> bool {
        self.timer_sync()
            .frame_timer
            .timer_tick_nano()
            .is_completed()
    }

    fn timer_sync(&mut self) -> &mut Self {
        if self.synced {
            return self;
        };
        self.frame_timer.reset_count();
        self.synced = true;
        self
    }
}
