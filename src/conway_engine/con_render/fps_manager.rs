use asca_timer::Timer;

#[derive(Debug)]
pub struct FrameRateManager {
    pub frame_timer: Timer,
    frame_counter: u64,
}

impl Default for FrameRateManager {
    //Will create the fps_manager with a 30Hz rate
    fn default() -> Self {
        let duration = std::time::Duration::from_millis(1000);
        let timer = Timer::new(duration, std::time::Instant::now(), true);
        Self {
            frame_timer: timer,
            frame_counter: 0,
        }
    }
}

impl FrameRateManager {
    pub fn new(fps_ms: u32) -> Self {
        let duration = std::time::Duration::from_millis(fps_ms.into());
        let timer = Timer::new(duration, std::time::Instant::now(), true);
        Self {
            frame_timer: timer,
            frame_counter: 0,
        }
    }
    pub fn should_draw(&mut self) -> bool {
        //self.frame_timer.timer_tick();
        self.frame_timer.timer_tick().timer_status()
    }
}
