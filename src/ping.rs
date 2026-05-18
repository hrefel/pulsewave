use std::net::IpAddr;
use std::time::Duration;

use surge_ping::{Client, Config, PingIdentifier, PingSequence};

use crate::app::{PingEvent, PingResult};

async fn resolve_host(host: &str) -> anyhow::Result<IpAddr> {
    let addr = tokio::net::lookup_host(format!("{}:0", host))
        .await?
        .next()
        .map(|a| a.ip())
        .ok_or_else(|| anyhow::anyhow!("could not resolve host: {}", host))?;
    Ok(addr)
}

pub async fn ping_loop(
    host: &str,
    interval: Duration,
    timeout: Duration,
    tx: tokio::sync::mpsc::Sender<PingEvent>,
) {
    let addr = match resolve_host(host).await {
        Ok(a) => a,
        Err(e) => {
            let _ = tx
                .send(PingEvent {
                    host: host.to_string(),
                    result: PingResult::Error(e.to_string()),
                })
                .await;
            return;
        }
    };

    let client = match Client::new(&Config::default()) {
        Ok(c) => c,
        Err(e) => {
            let _ = tx
                .send(PingEvent {
                    host: host.to_string(),
                    result: PingResult::Error(e.to_string()),
                })
                .await;
            return;
        }
    };

    let mut pinger = client.pinger(addr, PingIdentifier::from(0)).await;
    pinger.timeout(timeout);

    let payload = [0u8; 56];
    let mut seq: u16 = 0;

    loop {
        match pinger.ping(PingSequence::from(seq), &payload).await {
            Ok((_, dur)) => {
                let latency_ms = dur.as_secs_f64() * 1000.0;
                let _ = tx
                    .send(PingEvent {
                        host: host.to_string(),
                        result: PingResult::Ok { latency_ms },
                    })
                    .await;
            }
            Err(_) => {
                let _ = tx
                    .send(PingEvent {
                        host: host.to_string(),
                        result: PingResult::Timeout,
                    })
                    .await;
            }
        }
        seq = seq.wrapping_add(1);
        tokio::time::sleep(interval).await;
    }
}
