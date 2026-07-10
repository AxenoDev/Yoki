use std::sync::Arc;
use std::time::{Duration, Instant};

use sysinfo::{ProcessesToUpdate, System};
use tokio::time::MissedTickBehavior;

use crate::server::performance::{PerformanceMonitor, SystemSnapshot, TICKS_PER_SECOND};
use crate::ServerState;

const TICK_INTERVAL: Duration = Duration::from_millis(1000 / TICKS_PER_SECOND as u64);
const SNAPSHOT_EVERY_TICKS: u32 = TICKS_PER_SECOND;

pub fn spawn_tick_loop(server_state: Arc<ServerState>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(TICK_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        let mut system = System::new();
        let pid = sysinfo::get_current_pid().ok();
        let mut tick_count = 0u32;

        loop {
            interval.tick().await;
            let tick_start = Instant::now();

            // TODO: run the game tick here.

            tick_count = tick_count.wrapping_add(1);
            if tick_count.is_multiple_of(SNAPSHOT_EVERY_TICKS) {
                refresh_snapshot(server_state.performance(), &mut system, pid);
            }

            server_state.performance().record_tick(tick_start.elapsed());
        }
    });
}

fn refresh_snapshot(monitor: &PerformanceMonitor, system: &mut System, pid: Option<sysinfo::Pid>) {
    system.refresh_cpu_usage();
    system.refresh_memory();
    if let Some(pid) = pid {
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    }

    let cpu_count = system.cpus().len().max(1);
    let process = pid.and_then(|pid| system.process(pid));

    monitor.record_system_snapshot(SystemSnapshot {
        process_cpu_percent: process
            .map(|process| process.cpu_usage() / cpu_count as f32)
            .unwrap_or_default(),
        system_cpu_percent: system.global_cpu_usage(),
        cpu_count,
        process_memory_bytes: process.map(sysinfo::Process::memory).unwrap_or_default(),
        used_memory_bytes: system.used_memory(),
        total_memory_bytes: system.total_memory(),
    });
}
