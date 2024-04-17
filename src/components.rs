use web_time::{Duration, Instant};

pub fn to_display(duration: Duration) -> String {
    format!(
        "{:02}:{:02}",
        (duration.as_secs() / 60) % 60,
        duration.as_secs() % 60,
    )
}

pub fn to_display_h_m_s(duration: Duration) -> String {
    if duration.as_secs() >= 3600 {
        format!(
            "{:02}h{:02}m{:02}s",
            (duration.as_secs() / 3600) % 3600,
            (duration.as_secs() / 60) % 60,
            duration.as_secs() % 60,
        )
    } else {
        format!(
            "{:02}m{:02}s",
            (duration.as_secs() / 60) % 60,
            duration.as_secs() % 60,
        )
    }
}

pub struct Timespan {
    pub elapsed: Duration,
    pub start: Option<Instant>,
}

impl Timespan {
    pub fn new() -> Self {
        Self {
            elapsed: Duration::new(0, 0),
            start: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        if self.is_running() {
            self.elapsed = self.elapsed.saturating_add(Duration::new(
                Instant::now()
                    .saturating_duration_since(self.start.unwrap())
                    .as_secs(),
                0,
            ));
            self.start = None;
        }
    }

    pub fn reset(&mut self) {
        // TODO : call new instead
        self.elapsed = Duration::new(0, 0);
        self.start = None;
    }

    pub fn elapsed(&self) -> Duration {
        if self.start.is_some() {
            self.elapsed + Instant::now().saturating_duration_since(self.start.unwrap())
        } else {
            self.elapsed
        }
    }

    pub fn is_running(&self) -> bool {
        self.start.is_some()
    }
}

impl Default for Timespan {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Speech {
    pub duration: Duration,
    pub category: String,
}

impl Speech {
    pub fn new() -> Self {
        Self {
            duration: Duration::new(0, 0),
            category: String::new(),
        }
    }

    pub fn export_to_csv(&self) -> String {
        format!("{},\"{}\"", self.duration.as_secs(), self.category)
    }
}

impl Default for Speech {
    fn default() -> Self {
        Self::new()
    }
}
