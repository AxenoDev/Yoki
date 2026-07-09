use std::time::Duration;

use crate::commands::{Command, CommandContext, CommandError};
use crate::server::performance::{CpuUsage, PerformanceMonitor, TickStats, TICKS_PER_SECOND};

pub struct TpsCommand;

impl Command for TpsCommand {
    fn name(&self) -> &'static str {
        "tps"
    }

    fn description(&self) -> &'static str {
        "Shows server performance (TPS, tick times, CPU)"
    }

    fn execute(&self, ctx: &mut CommandContext<'_>) -> Result<(), CommandError> {
        let monitor = ctx.server_state.performance();

        ctx.reply("§7TPS from last 5s, 10s, 1m, 5m, 15m:".to_string());
        ctx.reply(format!(" {}", tps_line(monitor)));
        ctx.reply("".trim_end());
        ctx.reply("§7Tick durations (min/med/95%ile/max ms) from last 10s, 1m:".to_string());
        ctx.reply(format!(" {}", tick_durations_line(monitor)));
        ctx.reply("".trim_end());
        ctx.reply("§7CPU usage from last 10s, 1m, 15m:".to_string());
        ctx.reply(format!(
            " {}  §8(system)",
            cpu_line(monitor, |usage| usage.system)
        ));
        ctx.reply(format!(
            " {}  §8(process)",
            cpu_line(monitor, |usage| usage.process)
        ));

        Ok(())
    }
}

const TPS_WINDOWS: [Duration; 5] = [
    Duration::from_secs(5),       // 5 seconds
    Duration::from_secs(10),      // 10 seconds
    Duration::from_secs(60),      // 1 minutes
    Duration::from_secs(5 * 60),  // 5 minutes
    Duration::from_secs(15 * 60), // 15 minutes
];

fn tps_line(monitor: &PerformanceMonitor) -> String {
    TPS_WINDOWS
        .iter()
        .map(|window| colored_tps(monitor.tps(*window)))
        .collect::<Vec<_>>()
        .join("§7, ")
}

fn tick_durations_line(monitor: &PerformanceMonitor) -> String {
    [Duration::from_secs(10), Duration::from_secs(60)]
        .iter()
        .map(|window| match monitor.tick_durations(*window) {
            Some(stats) => format_tick_stats(&stats),
            None => "§8-".to_string(),
        })
        .collect::<Vec<_>>()
        .join("§7;  ")
}

fn cpu_line(monitor: &PerformanceMonitor, pick: impl Fn(&CpuUsage) -> f32) -> String {
    [
        Duration::from_secs(10),
        Duration::from_secs(60),
        Duration::from_secs(15 * 60),
    ]
    .iter()
    .map(|window| match monitor.cpu_usage(*window) {
        Some(usage) => colored_percent(pick(&usage)),
        None => "§8-".to_string(),
    })
    .collect::<Vec<_>>()
    .join("§7, ")
}

fn colored_tps(tps: f64) -> String {
    let star = if tps >= f64::from(TICKS_PER_SECOND) - 0.05 {
        "*"
    } else {
        ""
    };
    let color = if tps >= 18.0 {
        "§a"
    } else if tps >= 15.0 {
        "§e"
    } else {
        "§c"
    };
    format!("{color}{star}{tps:.1}")
}

fn format_tick_stats(stats: &TickStats) -> String {
    [stats.min, stats.median, stats.p95, stats.max]
        .iter()
        .map(|value| colored_tick_ms(*value))
        .collect::<Vec<_>>()
        .join("§7/")
}

fn colored_tick_ms(ms: f64) -> String {
    let budget = 1000.0 / f64::from(TICKS_PER_SECOND);
    let color = if ms <= budget / 2.0 {
        "§a"
    } else if ms <= budget {
        "§e"
    } else {
        "§c"
    };
    format!("{color}{ms:.1}")
}

fn colored_percent(percent: f32) -> String {
    let color = if percent < 50.0 {
        "§a"
    } else if percent < 80.0 {
        "§e"
    } else {
        "§c"
    };
    format!("{color}{percent:.0}%")
}
