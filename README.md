<div align="center">

# ◈ PULSEWAVE

**Real-time multi-host ping monitor with cyberpunk TUI**

[![CI](https://github.com/hrefel/pulsewave/actions/workflows/release.yml/badge.svg)](https://github.com/hrefel/pulsewave/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/pulsewave.svg)](https://crates.io/crates/pulsewave)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

```
╭──────────────────────────────────────────────────────────────────────────╮
│  ◈ PULSEWAVE v0.1.0    ↑ 2.3 MB/s    ↓ 15.1 MB/s    ▪ Total: 228.4 MB │
╰──────────────────────────────────────────────────────────────────────────╯
╭──────────────────────────────────────────────────────────────────────────╮
│                                                                          │
│          Latency Chart (braille line chart)                              │
│          One colored line per host — real-time update                    │
│                                                                          │
╰──────────────────────────────────────────────────────────────────────────╯
╭─ HOST STATISTICS ────────────────────────────────────────────────────────╮
│  Host            Cur    Min    Max    Avg    Loss    Jitter   Trend      │
│  ● google.com    12ms   8ms   45ms   15ms   0.0%    2.1ms    ▁▂▃▅▇▆▃▂▁  │
│  ● cloudflare    8ms    3ms   32ms   10ms   0.0%    1.3ms    ▁▂▃▅▇▆▃▂▁  │
│  ● github.com    45ms  12ms  230ms   52ms   2.1%    5.4ms    ▁▂▃▅▇▆▃▂▁  │
╰──────────────────────────────────────────────────────────────────────────╯
  [q]uit  [d]stats  [↑↓]scroll  [±]interval  [c]lear
```

## Features

- **Multi-host ping** — Monitor multiple servers simultaneously with color-coded lines
- **Real-time braille chart** — Smooth line charts using braille characters, updated live
- **Internet usage tracker** — Live upload/download speed (MB/s) + cumulative session total in header bar
- **Stats table** — Toggle detailed view with min/max/avg latency, packet loss %, jitter, and sparkline trends
- **Cyberpunk neon theme** — Dark background with neon cyan/magenta/yellow/green palette and rounded borders
- **Color-coded latency** — Green (<50ms) → Yellow (<150ms) → Red (>150ms)
- **Single binary** — Zero dependencies, ~1MB, just download and run

## Install

### Linux & macOS

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/hrefel/pulsewave/releases/latest/download/pulsewave-installer.sh | sh
```

### Windows

```powershell
powershell -c "irm https://github.com/hrefel/pulsewave/releases/latest/download/pulsewave-installer.ps1 | iex"
```

### Homebrew

```bash
brew install hrefel/tap/pulsewave
```

### Cargo

```bash
cargo install pulsewave
```

### Download

Download the latest binary for your platform from the [Releases](https://github.com/hrefel/pulsewave/releases) page.

## Usage

```bash
# Ping multiple hosts
pulsewave google.com cloudflare.com 1.1.1.1

# Custom ping interval (0.5 seconds)
pulsewave -i 0.5 google.com

# Custom timeout (2000ms) and max chart points (200)
pulsewave -W 2000 -n 200 google.com github.com
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-i, --interval` | `1.0` | Ping interval in seconds |
| `-W, --timeout` | `1000` | Ping timeout in milliseconds |
| `-n, --count` | `120` | Max data points in chart |
| `-h, --help` | | Print help |
| `-V, --version` | | Print version |

### Controls

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `d` | Toggle stats table |
| `↑` / `↓` | Scroll through hosts |
| `+` / `-` | Adjust ping interval |
| `c` | Clear chart data |

### Permissions

Pulsewave uses raw ICMP sockets for ping. On Linux, grant the capability:

```bash
sudo setcap cap_net_raw+ep $(which pulsewave)
```

Or run with `sudo`.

## Built With

- [Rust](https://www.rust-lang.org/) — Performance & safety
- [ratatui](https://ratatui.rs/) — Terminal UI framework
- [surge-ping](https://github.com/kolapapa/surge-ping) — Async ICMP ping
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — Network I/O stats
- [clap](https://clap.rs/) — CLI argument parser
- [tokio](https://tokio.rs/) — Async runtime

## License

[MIT](LICENSE)
