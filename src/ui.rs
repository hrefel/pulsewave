use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Borders, Cell, Chart, Dataset, GraphType, Paragraph, Row, Table},
    Frame,
};

use crate::app::AppState;
use crate::theme;

pub fn render(app: &mut AppState, frame: &mut Frame) {
    let bg = Block::default().style(Style::default().bg(theme::BG));
    frame.render_widget(bg, frame.area());

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(app, frame, main[0]);
    render_status(app, frame, main[1]);
    render_footer(frame, main[3]);

    if app.show_stats {
        let body = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(main[1]);
        render_chart(app, frame, body[0]);
        render_stats(app, frame, body[1]);
    } else {
        render_chart(app, frame, main[2]);
    }
}

fn render_header(app: &AppState, frame: &mut Frame, area: Rect) {
    let speed_fmt = |v: f64| {
        if v >= 1.0 {
            format!("{:.1} MB/s", v)
        } else {
            format!("{:.1} KB/s", v * 1000.0)
        }
    };

    let total_fmt = |v: f64| {
        if v >= 1.0 {
            format!("{:.1} MB", v)
        } else {
            format!("{:.0} KB", v * 1000.0)
        }
    };

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " ◈ PULSEWAVE ",
            Style::default()
                .fg(theme::TITLE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("v0.1.0", Style::default().fg(theme::FOOTER_KEY)),
        Span::raw("   "),
        Span::styled(
            "↑",
            Style::default()
                .fg(theme::UPLOAD)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {} ", speed_fmt(app.net.tx_speed)),
            Style::default().fg(theme::UPLOAD),
        ),
        Span::styled(
            "↓",
            Style::default()
                .fg(theme::DOWNLOAD)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {} ", speed_fmt(app.net.rx_speed)),
            Style::default().fg(theme::DOWNLOAD),
        ),
        Span::styled(
            " ▪ ",
            Style::default()
                .fg(theme::TOTAL)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(
                "Total: ↑{} ↓{}",
                total_fmt(app.net.total_tx_mb),
                total_fmt(app.net.total_rx_mb)
            ),
            Style::default().fg(theme::TOTAL),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER)),
    );

    frame.render_widget(header, area);
}

fn render_status(app: &AppState, frame: &mut Frame, area: Rect) {
    let cond = app.network_condition();

    let total_count: u64 = app.hosts.iter().map(|h| h.count).sum();
    let total_lost: u64 = app.hosts.iter().map(|h| h.lost).sum();
    let total_sum: f64 = app.hosts.iter().map(|h| h.sum_ms).sum();
    let overall_avg = if total_count > 0 {
        total_sum / total_count as f64
    } else {
        0.0
    };
    let overall_loss = if total_count > 0 {
        total_lost as f64 / total_count as f64 * 100.0
    } else {
        0.0
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" ◈ {} ", cond.status),
            Style::default()
                .fg(cond.color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("— {} ", cond.hint),
            Style::default().fg(cond.color),
        ),
        Span::raw("  "),
        Span::styled(
            format!("Avg: {:.1}ms", overall_avg),
            Style::default().fg(theme::FOOTER_KEY),
        ),
        Span::raw("  "),
        Span::styled(
            format!("Loss: {:.1}%", overall_loss),
            Style::default().fg(theme::FOOTER_KEY),
        ),
        Span::raw("  "),
        Span::styled(
            format!("Hosts: {}", app.hosts.len()),
            Style::default().fg(theme::FOOTER_KEY),
        ),
    ]))
    .style(Style::default().bg(theme::BG));

    frame.render_widget(status, area);
}

fn render_chart(app: &AppState, frame: &mut Frame, area: Rect) {
    let host_data: Vec<(usize, Vec<(f64, f64)>)> = app
        .hosts
        .iter()
        .enumerate()
        .filter(|(_, host)| !host.data.is_empty())
        .map(|(i, host)| {
            let data: Vec<(f64, f64)> = host
                .data
                .iter()
                .enumerate()
                .map(|(idx, (_, y))| (idx as f64, *y))
                .collect();
            (i, data)
        })
        .collect();

    let mut y_max = 100.0_f64;
    for (_, data) in &host_data {
        if let Some(&(_, max_y)) = data
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        {
            y_max = y_max.max(max_y);
        }
    }

    let datasets: Vec<Dataset> = host_data
        .iter()
        .map(|(i, data)| {
            Dataset::default()
                .name(app.hosts[*i].host.as_str())
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(theme::host_color(*i)))
                .data(data)
        })
        .collect();

    let y_max_chart = y_max * 1.2 + 5.0;
    let x_max = app
        .hosts
        .iter()
        .map(|h| h.data.len())
        .max()
        .unwrap_or(1) as f64;

    let x_axis = Axis::default()
        .style(Style::default().fg(theme::AXIS))
        .bounds([0.0, x_max.max(1.0)]);

    let y_axis = Axis::default()
        .title(Span::styled(
            "ms",
            Style::default().fg(theme::TITLE),
        ))
        .style(Style::default().fg(theme::AXIS))
        .bounds([0.0, y_max_chart])
        .labels(vec![
            Span::raw("0"),
            Span::raw(format!("{:.0}", y_max_chart / 2.0)),
            Span::raw(format!("{:.0}", y_max_chart)),
        ]);

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Line::from(Span::styled(
                    " LATENCY ",
                    Style::default()
                        .fg(theme::TITLE)
                        .add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::BORDER)),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart, area);
}

fn render_stats(app: &AppState, frame: &mut Frame, area: Rect) {
    let header = Row::new(vec![
        Cell::from("Host").style(
            Style::default()
                .fg(theme::TITLE)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("Cur"),
        Cell::from("Min"),
        Cell::from("Max"),
        Cell::from("Avg"),
        Cell::from("Loss"),
        Cell::from("Jitter"),
        Cell::from("Trend"),
    ])
    .height(1)
    .bottom_margin(1);

    let rows: Vec<Row> = app
        .hosts
        .iter()
        .enumerate()
        .map(|(i, host)| {
            let cur = format_latency(host.current_ms);
            let min = if host.min_ms == f64::MAX {
                "  -  ".to_string()
            } else {
                format_latency(host.min_ms)
            };
            let max = format_latency(host.max_ms);
            let avg = format_latency(host.avg_ms());
            let loss = format!("{:.1}%", host.loss_pct());
            let jitter = format!("{:.1}ms", host.jitter());
            let trend = sparkline_str(&host.sparkline);

            Row::new(vec![
                Cell::from(format!("● {}", host.host))
                    .style(Style::default().fg(theme::host_color(i))),
                Cell::from(cur).style(Style::default().fg(theme::latency_color(host.current_ms))),
                Cell::from(min).style(Style::default().fg(theme::latency_color(host.min_ms))),
                Cell::from(max).style(Style::default().fg(theme::latency_color(host.max_ms))),
                Cell::from(avg).style(Style::default().fg(theme::latency_color(host.avg_ms()))),
                Cell::from(loss).style(Style::default().fg(if host.loss_pct() > 0.0 {
                    theme::BAD
                } else {
                    theme::GOOD
                })),
                Cell::from(jitter),
                Cell::from(trend).style(Style::default().fg(theme::host_color(i))),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(Line::from(Span::styled(
                " HOST STATISTICS ",
                Style::default()
                    .fg(theme::TITLE)
                    .add_modifier(Modifier::BOLD),
            )))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER)),
    );

    frame.render_widget(table, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [q]uit  ", Style::default().fg(theme::FOOTER_KEY)),
        Span::styled("[d]stats  ", Style::default().fg(theme::FOOTER_KEY)),
        Span::styled("[↑↓]scroll  ", Style::default().fg(theme::FOOTER_KEY)),
        Span::styled("[±]interval  ", Style::default().fg(theme::FOOTER_KEY)),
        Span::styled("[c]lear", Style::default().fg(theme::FOOTER_KEY)),
    ]))
    .style(Style::default().bg(theme::BG));

    frame.render_widget(footer, area);
}

fn format_latency(ms: f64) -> String {
    if ms < 1.0 {
        format!("{:.2}ms", ms)
    } else {
        format!("{:.1}ms", ms)
    }
}

fn sparkline_str(data: &[u64]) -> String {
    const CHARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let max = data.iter().copied().max().unwrap_or(1);
    data.iter()
        .map(|&v| {
            let idx = if max == 0 {
                0
            } else {
                ((v as f64 / max as f64) * (CHARS.len() - 1) as f64).round() as usize
            };
            CHARS[idx.min(CHARS.len() - 1)]
        })
        .collect()
}
