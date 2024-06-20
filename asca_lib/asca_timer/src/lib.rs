//! # asca_Timer
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
//!             timer.timer_tick_mili();
//!             let result = timer.try_execute(|| {
//!                 println!("It works");
//!                 });
//!
//!             if timer.is_completed() == true {
//!                 println!("{}", timer);
//!                 break;
//!             }
//!         }
//!     }

/// ### The meat and potatoes of this crate. Holds all of the data required for this crate to
/// function.
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

//TODO: Add a type-state pattern so that the user only needs to call timer.tick(), and not specify
//the accuracy
#[derive(Debug)]
pub struct Timer {
    timer_duration: Duration,
    time_elapsed: Instant,
    counting: bool,
    completed: bool,
    repeating: bool,
    completions: u64,
    executions: u32,
}

/// ## These implentations are the basic utility functions of this crate.
impl Timer {
    pub fn new(timer_duration: Duration, timer_start: Instant, repeating: bool) -> Self {
        Self {
            timer_duration,
            time_elapsed: timer_start,
            repeating,
            completed: false,
            completions: 0,
            executions: 0,
            counting: true,
        }
    }

    /// Checkes the completion status of the timer.
    ///
    /// Returns a bool inidcating completion status.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// //--snip--
    /// if timer.is_completed() == true {
    ///     println!("Hello World");
    /// }
    /// ```
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Checkes the status of the timer.
    ///
    /// Returns a bool indicating if the timer is finished counting, and is completed.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// //--snip--
    /// if timer.is_finished() == true {
    ///     println!("Hello World");
    /// }
    /// ```
    pub fn is_finished(&self) -> bool {
        if !self.repeating && self.completed && !self.counting {
            return true;
        }
        return false;
    }

    /// Returns the number of times the timer has finished counting
    /// # Example
    ///
    /// ```rust,ignore
    /// loop {
    /// //--snip--
    ///     if timer.timer_completions() == 10 {
    ///         println!("Timer completed {} times!", timer.timer_completions());
    ///         break;
    ///     }
    /// }
    /// ```
    pub fn timer_completions(&self) -> u64 {
        self.completions
    }

    /// Will consume the timer, and call drop() on it.
    /// # Example
    ///
    /// ```rust,ignore
    /// loop {
    /// //--snip--
    ///     if timer.timer_completions() == 10 {
    ///         println!("Timer completed!");
    ///         timer.timer_destroy();
    ///         break;
    ///     }
    /// }
    /// ```
    pub fn timer_destroy(self) {
        drop(self);
    }

    pub fn reset_count(&mut self) -> &mut Self {
        self.time_elapsed = Instant::now();
        self
    }
}

/// ## The main function of this libary.
///
/// These functions **must** be called every loop,
/// otherwise the timer will *not* be able to keep track of time.
///
/// This can be called either at the begining of a loop or a the end of one.
/// It's specific placement doesn't matter as long as it is consistant.
///
/// The only difference between each implementation is the accuracy that is used to measure the
/// length of time. Otherwise, they all perform the same operations.
///
/// # Example
/// ```rust,ignore
///     loop {
///         //--snip--
///         timer.timer_tick_mili();
///     }
impl Timer {
    pub fn timer_tick_mili(&mut self) -> &mut Self {
        if self.counting == false {
            return self;
        }

        if self.check_timer_mili() == false {
            self.completed = false;
            return self;
        } else {
            if self.repeating == false {
                self.timer_complete();
                return self;
            } else {
                self.time_elapsed = Instant::now();
                self.completed = true;
                self.completions += 1;
                return self;
            }
        }
    }

    pub fn timer_tick_micro(&mut self) -> &mut Self {
        if self.counting == false {
            return self;
        }

        if self.check_timer_micro() == false {
            self.completed = false;
            return self;
        } else {
            if self.repeating == false {
                self.timer_complete();
                return self;
            } else {
                self.time_elapsed = Instant::now();
                self.completed = true;
                self.completions += 1;
                return self;
            }
        }
    }

    pub fn timer_tick_nano(&mut self) -> &mut Self {
        if self.counting == false {
            return self;
        }

        if self.check_timer_nano() == false {
            self.completed = false;
            return self;
        } else {
            if self.repeating == false {
                self.timer_complete();
                return self;
            } else {
                self.time_elapsed = Instant::now();
                self.completed = true;
                self.completions += 1;
                return self;
            }
        }
    }
}

/// ## These implementations extend the functionality of the timer.
impl Timer {
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
        if !self.completed && self.check_timer_mili() {
            anon_func();
            self.completed = true;
            self.executions += 1;
            return Ok(());
        } else {
            Err(())
        }
    }
}

impl Timer {
    fn timer_complete(&mut self) -> &mut Self {
        self.completions += 1;
        self.counting = false;
        self.completed = true;
        self
    }
    fn check_timer_mili(&self) -> bool {
        self.time_elapsed
            .elapsed()
            .as_millis()
            .eq(&self.timer_duration.as_millis())
    }

    fn check_timer_micro(&mut self) -> bool {
        if self.time_elapsed.elapsed().as_millis() == 0 {
            return false;
        }
        if self.time_elapsed.elapsed().as_micros() / self.timer_duration.as_micros() == 1 {
            return true;
        } else {
            return false;
        }
    }

    fn check_timer_nano(&mut self) -> bool {
        if self.time_elapsed.elapsed().as_nanos() == 0 {
            return false;
        }
        if self.time_elapsed.elapsed().as_nanos() / self.timer_duration.as_nanos() == 1 {
            return true;
        } else {
            return false;
        }
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
            ",
            self.timer_duration.as_millis(),
            self.time_elapsed.elapsed().as_millis(),
            self.completed,
            self.repeating,
            self.completions,
            self.executions,
        )
    }
}
