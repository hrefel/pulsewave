mod app;
mod ping;
mod theme;
mod ui;

use std::time::Duration;

use anyhow::Result;
use app::{AppState, PingEvent};
use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Parser)]
#[command(name = "pulsewave", version, about = "Real-time multi-host ping monitor with cyberpunk TUI")]
struct Cli {
    #[arg(required = true)]
    hosts: Vec<String>,

    #[arg(short, long, default_value_t = 1.0)]
    interval: f64,

    #[arg(short = 'W', long, default_value_t = 1000)]
    timeout: u64,

    #[arg(short = 'n', long, default_value_t = 120)]
    count: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let interval = Duration::from_secs_f64(cli.interval);
    let timeout = Duration::from_millis(cli.timeout);

    let (ping_tx, mut ping_rx) = tokio::sync::mpsc::channel::<PingEvent>(256);

    for host in &cli.hosts {
        let tx = ping_tx.clone();
        let host = host.clone();
        tokio::spawn(async move {
            ping::ping_loop(&host, interval, timeout, tx).await;
        });
    }
    drop(ping_tx);

    let mut terminal = ratatui::init();
    let mut app = AppState::new(cli.hosts, cli.count);

    let (key_tx, mut key_rx) = tokio::sync::mpsc::channel::<KeyCode>(32);
    std::thread::spawn(move || {
        loop {
            if crossterm::event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(key) = crossterm::event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        if key_tx.blocking_send(key.code).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    let tick_rate = Duration::from_millis(200);
    let mut tick = tokio::time::interval(tick_rate);

    loop {
        terminal.draw(|frame| app.render(frame))?;

        tokio::select! {
            Some(key) = key_rx.recv() => {
                match key {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('d') => app.toggle_stats(),
                    KeyCode::Up => app.scroll_up(),
                    KeyCode::Down => app.scroll_down(),
                    KeyCode::Char('+') => {}
                    KeyCode::Char('-') => {}
                    KeyCode::Char('c') => app.clear_chart(),
                    _ => {}
                }
            }
            Some(event) = ping_rx.recv() => {
                app.handle_event(event);
            }
            _ = tick.tick() => {
                app.on_tick();
            }
        }
    }

    ratatui::restore();
    Ok(())
}
