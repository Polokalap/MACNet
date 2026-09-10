use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::AppState;
use crate::logger::{info, warn};

pub async fn init(state: AppState) {

    let mut reader = BufReader::new(tokio::io::stdin()).lines();

    loop {

        print!("> ");
        io::stdout().flush().unwrap();

        let line = match reader.next_line().await {
            Ok(Some(line)) => line,
            Ok(None) => break, // stdin closed (EOF)
            Err(_) => {
                warn("Failed to read line, please open an issue on our github!").await;
                continue;
            }
        };

        let input = line.trim();

        if input.is_empty() {

            continue;

        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("").to_lowercase();
        let args: Vec<&str> = parts.collect();

        match command.as_str() {
            "stats" => {

                let now = SystemTime::now();
                let ms = now
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();

                let elapsed_time = ms - state.start;
                let elapsed = format_duration(elapsed_time);

                info(format!("Running for {}!", elapsed).as_str()).await;

            }
            "clear" => {

                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();

            }
            _ => {}
        }

    }

}

fn format_duration(elapsed_ms: u128) -> String {

    if elapsed_ms < 1000 {

        return format!("{}ms", elapsed_ms);

    }

    let total_secs = elapsed_ms / 1000;

    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let mut parts = Vec::new();

    if days > 0 {

        parts.push(format!("{}d", days));

    }

    if hours > 0 {

        parts.push(format!("{}h", hours));

    }

    if minutes > 0 {

        parts.push(format!("{}m", minutes));

    }
    if seconds > 0 || parts.is_empty() {

        parts.push(format!("{}s", seconds));

    }

    parts.join(" ")

}