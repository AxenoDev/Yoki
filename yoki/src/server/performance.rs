use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const TICKS_PER_SECOND: u32 = 20;
const HISTORY_WINDOW: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Clone, Copy)]
struct TickSample {
    at: Instant,
    duration: Duration,
}

#[derive(Debug, Clone, Copy)]
struct CpuSample {
    at: Instant,
    process: f32,
    system: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickStats {
    pub min: f64,
    pub median: f64,
    pub p95: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CpuUsage {
    pub process: f32,
    pub system: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemSnapshot {
    pub process_cpu_percent: f32,
    pub system_cpu_percent: f32,
    pub cpu_count: usize,
    pub process_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub total_memory_bytes: u64,
}

#[derive(Debug)]
pub struct PerformanceMonitor {
    started: Instant,
    ticks: Mutex<VecDeque<TickSample>>,
    cpu_samples: Mutex<VecDeque<CpuSample>>,
    snapshot: Mutex<SystemSnapshot>,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            ticks: Mutex::new(VecDeque::new()),
            cpu_samples: Mutex::new(VecDeque::new()),
            snapshot: Mutex::new(SystemSnapshot::default()),
        }
    }
}

impl PerformanceMonitor {
    pub fn uptime(&self) -> Duration {
        self.started.elapsed()
    }

    pub fn record_tick(&self, duration: Duration) {
        let mut ticks = self.ticks.lock().expect("ticks lock poisoned");
        let now = Instant::now();
        ticks.push_back(TickSample { at: now, duration });
        while let Some(oldest) = ticks.front() {
            if now.duration_since(oldest.at) > HISTORY_WINDOW {
                ticks.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn tps(&self, window: Duration) -> f64 {
        let effective = window.min(self.uptime()).max(Duration::from_secs(1));
        let cutoff = Instant::now() - effective;
        let ticks = self.ticks.lock().expect("ticks lock poisoned");
        let count = ticks.iter().filter(|sample| sample.at >= cutoff).count();
        (count as f64 / effective.as_secs_f64()).min(TICKS_PER_SECOND as f64)
    }

    pub fn tick_durations(&self, window: Duration) -> Option<TickStats> {
        let cutoff = Instant::now() - window.min(self.uptime());
        let ticks = self.ticks.lock().expect("ticks lock poisoned");
        let mut millis: Vec<f64> = ticks
            .iter()
            .filter(|sample| sample.at >= cutoff)
            .map(|sample| sample.duration.as_secs_f64() * 1000.0)
            .collect();
        if millis.is_empty() {
            return None;
        }
        millis.sort_by(|left, right| left.total_cmp(right));

        let percentile = |p: f64| {
            let index = ((millis.len() as f64 * p).ceil() as usize).saturating_sub(1);
            millis[index.min(millis.len() - 1)]
        };
        Some(TickStats {
            min: millis[0],
            median: percentile(0.50),
            p95: percentile(0.95),
            max: millis[millis.len() - 1],
        })
    }

    pub fn cpu_usage(&self, window: Duration) -> Option<CpuUsage> {
        let cutoff = Instant::now() - window.min(self.uptime());
        let samples = self.cpu_samples.lock().expect("cpu samples lock poisoned");
        let recent: Vec<_> = samples
            .iter()
            .filter(|sample| sample.at >= cutoff)
            .collect();
        if recent.is_empty() {
            return None;
        }
        let count = recent.len() as f32;
        Some(CpuUsage {
            process: recent.iter().map(|sample| sample.process).sum::<f32>() / count,
            system: recent.iter().map(|sample| sample.system).sum::<f32>() / count,
        })
    }

    pub fn snapshot(&self) -> SystemSnapshot {
        *self.snapshot.lock().expect("snapshot lock poisoned")
    }

    pub(crate) fn record_system_snapshot(&self, snapshot: SystemSnapshot) {
        let mut samples = self.cpu_samples.lock().expect("cpu samples lock poisoned");
        let now = Instant::now();
        samples.push_back(CpuSample {
            at: now,
            process: snapshot.process_cpu_percent,
            system: snapshot.system_cpu_percent,
        });
        while let Some(oldest) = samples.front() {
            if now.duration_since(oldest.at) > HISTORY_WINDOW {
                samples.pop_front();
            } else {
                break;
            }
        }
        drop(samples);

        *self.snapshot.lock().expect("snapshot lock poisoned") = snapshot;
    }
}
