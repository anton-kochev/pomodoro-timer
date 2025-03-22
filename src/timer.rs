use std::time::Instant;

enum IntervalKind {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

pub struct TimerResult {
    pub done: bool,
    pub minutes: u64,
    pub seconds: u64,
}

pub struct Timer {
    started_at: Instant,
    duration: u64,
}

impl Timer {
    pub fn new(timeout: u64) -> Self {
        Timer {
            started_at: Instant::now(),
            duration: timeout,
        }
    }

    pub fn elapsed(&self) -> TimerResult {
        let elapsed_seconds = self.started_at.elapsed().as_secs();
        let done = elapsed_seconds >= self.duration;
        let (minutes, seconds) = match done {
            false => (
                (self.duration - elapsed_seconds) / 60,
                (self.duration - elapsed_seconds) % 60,
            ),
            _ => (0, 0),
        };

        TimerResult {
            done,
            minutes,
            seconds,
        }
    }
}
