use std::collections::VecDeque;
use std::time::Duration;

use sysinfo::Networks;

pub struct PingEvent {
    pub host: String,
    pub result: PingResult,
}

pub enum PingResult {
    Ok { latency_ms: f64 },
    Timeout,
    Error(String),
}

pub struct HostState {
    pub host: String,
    pub data: VecDeque<(f64, f64)>,
    pub current_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub sum_ms: f64,
    pub count: u64,
    pub lost: u64,
    pub sparkline: Vec<u64>,
    max_points: usize,
}

impl HostState {
    pub fn new(host: String, max_points: usize) -> Self {
        Self {
            host,
            data: VecDeque::with_capacity(max_points),
            current_ms: 0.0,
            min_ms: f64::MAX,
            max_ms: 0.0,
            sum_ms: 0.0,
            count: 0,
            lost: 0,
            sparkline: Vec::new(),
            max_points,
        }
    }

    pub fn record(&mut self, latency_ms: f64) {
        let x = self.count as f64;
        if self.data.len() >= self.max_points {
            self.data.pop_front();
        }
        self.data.push_back((x, latency_ms));
        self.current_ms = latency_ms;
        if latency_ms < self.min_ms {
            self.min_ms = latency_ms;
        }
        if latency_ms > self.max_ms {
            self.max_ms = latency_ms;
        }
        self.sum_ms += latency_ms;
        self.count += 1;
        self.update_sparkline();
    }

    pub fn record_timeout(&mut self) {
        self.lost += 1;
        self.count += 1;
        if self.data.len() >= self.max_points {
            self.data.pop_front();
        }
        self.data.push_back((self.count as f64, 0.0));
        self.update_sparkline();
    }

    fn update_sparkline(&mut self) {
        self.sparkline = self
            .data
            .iter()
            .rev()
            .take(20)
            .rev()
            .map(|(_, y)| (*y * 10.0) as u64)
            .collect();
    }

    pub fn avg_ms(&self) -> f64 {
        if self.count > 0 {
            self.sum_ms / self.count as f64
        } else {
            0.0
        }
    }

    pub fn loss_pct(&self) -> f64 {
        if self.count > 0 {
            self.lost as f64 / self.count as f64 * 100.0
        } else {
            0.0
        }
    }

    pub fn jitter(&self) -> f64 {
        if self.data.len() < 2 {
            return 0.0;
        }
        let vals: Vec<f64> = self.data.iter().map(|(_, y)| *y).collect();
        let mut diffs = Vec::with_capacity(vals.len() - 1);
        for i in 1..vals.len() {
            diffs.push((vals[i] - vals[i - 1]).abs());
        }
        diffs.iter().sum::<f64>() / diffs.len() as f64
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.current_ms = 0.0;
        self.min_ms = f64::MAX;
        self.max_ms = 0.0;
        self.sum_ms = 0.0;
        self.count = 0;
        self.lost = 0;
        self.sparkline.clear();
    }
}

pub struct NetworkStats {
    pub rx_speed: f64,
    pub tx_speed: f64,
    pub total_rx_mb: f64,
    pub total_tx_mb: f64,
    networks: Networks,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            rx_speed: 0.0,
            tx_speed: 0.0,
            total_rx_mb: 0.0,
            total_tx_mb: 0.0,
            networks: Networks::new_with_refreshed_list(),
        }
    }

    pub fn refresh(&mut self, interval_secs: f64) {
        self.networks.refresh(true);
        let mut rx_bytes: u64 = 0;
        let mut tx_bytes: u64 = 0;
        for (_name, data) in &self.networks {
            rx_bytes += data.received();
            tx_bytes += data.transmitted();
        }
        if interval_secs > 0.0 {
            self.rx_speed = rx_bytes as f64 / interval_secs / 1_000_000.0;
            self.tx_speed = tx_bytes as f64 / interval_secs / 1_000_000.0;
        }
        self.total_rx_mb += rx_bytes as f64 / 1_000_000.0;
        self.total_tx_mb += tx_bytes as f64 / 1_000_000.0;
    }
}

pub struct AppState {
    pub hosts: Vec<HostState>,
    pub net: NetworkStats,
    pub show_stats: bool,
    pub selected: usize,
    pub tick_interval: Duration,
}

impl AppState {
    pub fn new(hosts: Vec<String>, max_points: usize) -> Self {
        let host_states = hosts
            .into_iter()
            .map(|h| HostState::new(h, max_points))
            .collect();
        Self {
            hosts: host_states,
            net: NetworkStats::new(),
            show_stats: false,
            selected: 0,
            tick_interval: Duration::from_millis(200),
        }
    }

    pub fn handle_event(&mut self, event: PingEvent) {
        if let Some(host) = self.hosts.iter_mut().find(|h| h.host == event.host) {
            match event.result {
                PingResult::Ok { latency_ms } => host.record(latency_ms),
                PingResult::Timeout => host.record_timeout(),
                PingResult::Error(_) => host.record_timeout(),
            }
        }
    }

    pub fn toggle_stats(&mut self) {
        self.show_stats = !self.show_stats;
    }

    pub fn scroll_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.selected < self.hosts.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn clear_chart(&mut self) {
        for h in &mut self.hosts {
            h.clear();
        }
    }

    pub fn on_tick(&mut self) {
        self.net.refresh(self.tick_interval.as_secs_f64());
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        crate::ui::render(self, frame);
    }
}
