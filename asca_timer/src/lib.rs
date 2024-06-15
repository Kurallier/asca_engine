//! # ASCA-Timer
//!
//! 'asca_timer' is a crate designed for the ASCA engine, although there may be other uses out
//! of it.
//!
//! This crate is intended to be simple and easy to implement, with the user only needing to call a
//! handful of functions.
//!
//! # Example
//! ```rust
//!     use std::time::{Duration, Instant};
//!     use asca_timer::Timer;
//!
//!     fn main() {
//!         let mut timer = Timer::new(Duration::from_millis(1000), Instant::now(), false);
//!
//!         loop {
//!             timer.timer_tick();
//!             let result = timer.try_execute(|| {
//!                 println!("It works");
//!                 });
//!
//!             if timer.timer_status() == true {
//!                 println!("{}", timer);
//!                 break;
//!             }
//!         }
//!     }

/// The structure that hold all of the data needed to calculate time.
use float_cmp::{ApproxEq, F64Margin};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Timer {
    timer_duration: Duration,
    time_elapsed: Instant,
    check_margin: F64Margin,
    counting: bool,
    completed: bool,
    repeating: bool,
    completions: u64,
    executions: u32,
}

#[allow(dead_code)]
impl Timer {
    pub fn new(timer_duration: Duration, timer_start: Instant, repeating: bool) -> Self {
        Self {
            timer_duration,
            time_elapsed: timer_start,
            check_margin: F64Margin {
                ulps: 5,
                epsilon: 0.000002,
            },
            repeating,
            completed: false,
            completions: 0,
            executions: 0,
            counting: true,
        }
    }

    //Internal function, chects the status of the timer
    fn check_timer(&self) -> bool {
        self.time_elapsed
            .elapsed()
            .as_millis()
            .eq(&self.timer_duration.as_millis())
    }

    fn check_timer_nano(&mut self) -> bool {
        if self.time_elapsed.elapsed().as_nanos().eq(&self.timer_duration.as_nanos()) {
            return true;
        } else {
            return false;
        }
    }

    /// The main function of this libary.
    ///
    /// This **must** be called every loop,
    /// otherwise the timer will *not* be able to keep track of time.
    ///
    /// This can be called either at the begining of a loop or a the end of one.
    /// It's specific placement doesn't matter as long as it is consistant.
    ///
    /// # Example
    /// ```rust,ignore
    ///     loop {
    ///         //--snip--
    ///         timer.timer_tick();
    ///     }
    pub fn timer_tick(&mut self) -> &mut Self {
        if self.check_timer() {
            if self.counting {
                self.completed = true;
                self.completions += 1;
                if self.repeating == true {
                    self.time_elapsed = Instant::now();
                } else {
                    self.counting = false;
                }
            } else {
                self.completed = false;
                self.counting = true;
            }
        }
        self
    }

    pub fn timer_tick_nano(&mut self) -> &mut Self {
        if self.check_timer() == true {
            if self.repeating == true {
                self.completed = false;
                self.time_elapsed = Instant::now();
            } else {
                self.completed = true;
                self.counting = false;
            }
            self.completions += 1;
        }
        self
    }

    /// Checkes the completion status of the timer
    ///
    /// Returns a bool inidcating completion status
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// //--snip--
    /// if timer.timer_status == true {
    ///     println!("Hello World");
    /// }
    /// ```
    pub fn timer_status(&mut self) -> bool {
        self.completed
    }

    pub fn timer_completions(&self) -> u64 {
        self.completions
    }

    /// Will try to execute any function that is passed in.
    ///
    /// Returns an Ok() if the function was able to execute the function,
    /// or it will return an Err() indicating that it was unable to exectute the function.
    ///
    /// # Example
    /// ```rust, ignore
    /// //--snip--
    /// let result = timer.try_execute(|| {
    ///     println!("It works");
    ///     });
    ///```
    pub fn try_execute<F: FnOnce()>(&mut self, anon_func: F) -> Result<(), ()> {
        if !self.completed && self.check_timer() {
            anon_func();
            self.completed = true;
            self.executions += 1;
            return Ok(());
        } else {
            Err(())
        }
    }

    pub fn num_completions(&self) -> u64 {
        self.completions
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
                Timer Duration: {}ms\n
                Time Elapsed: {}ms\n
                Timer Completion: {}\n
                Repeating Timer: {}\n
                Timer Completions: {}\n
                Timer Code Executions: {}\n
                Timer Calculation Margin {:?}
            ",
            self.timer_duration.as_millis(),
            self.time_elapsed.elapsed().as_millis(),
            self.completed,
            self.repeating,
            self.completions,
            self.executions,
            self.check_margin
        )
    }
}
