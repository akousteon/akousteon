use std::collections::VecDeque;

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

    pub fn start_or_stop(&mut self) {
        if !self.is_running() {
            self.start();
        } else {
            self.stop()
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

#[derive(Clone)]
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

pub type Order = VecDeque<usize>;
pub type Speaker = (String, String);
pub type Speakers = Vec<Speaker>;

pub trait TSpeakers {
    fn current_speaker(&self, order: &Order) -> Speaker;
    fn next_speaker(&self, order: &Order) -> Speaker;
    fn add_speaker(&mut self, speaker: Speaker);
    fn delete_speaker(&mut self, speaker: usize, order: &mut Order);
    fn get_speaker(&self, speaker: usize) -> Speaker;
    fn speaker_wants_to_speak(&self, speaker: usize, order: &mut Order);
    fn speaker_spoke(&self, order: &mut Order) -> Speaker;
}

impl TSpeakers for Speakers {
    fn current_speaker(&self, order: &Order) -> Speaker {
        match order.front() {
            Some(&s) => self
                .get(s)
                .unwrap_or(&(String::new(), String::new()))
                .clone(),
            None => (String::new(), String::new()),
        }
    }

    fn next_speaker(&self, order: &Order) -> Speaker {
        match order.get(1) {
            Some(&s) => self
                .get(s)
                .unwrap_or(&(String::new(), String::new()))
                .clone(),
            None => (String::new(), String::new()),
        }
    }

    fn add_speaker(&mut self, speaker: Speaker) {
        self.push(speaker);
    }

    fn delete_speaker(&mut self, speaker: usize, order: &mut Order) {
        self.remove(speaker);
        // Shift index in order
        order.retain_mut(|x| {
            if *x != speaker {
                if *x > speaker {
                    *x -= 1;
                }
                true
            } else {
                false
            }
        });
    }

    fn get_speaker(&self, speaker: usize) -> Speaker {
        match self.get(speaker) {
            Some(s) => s.clone(),
            None => (String::new(), String::new()),
        }
    }

    fn speaker_wants_to_speak(&self, speaker: usize, order: &mut Order) {
        order.push_back(speaker);
    }

    fn speaker_spoke(&self, order: &mut Order) -> Speaker {
        match order.pop_front() {
            Some(s) => self
                .get(s)
                .unwrap_or(&(String::new(), String::new()))
                .clone(),
            None => (String::new(), String::new()),
        }
    }
}
