use std::io::Write;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::response::IntoResponse;
use rustyline_async::{Readline, ReadlineEvent};
use toml::macros::insert_toml;
use crate::{logger, AppState, STATE};
use crate::logger::{info, warn};

pub async fn init(state: AppState) {

    let (mut rl, mut stdout) = Readline::new("> ".to_owned()).expect("Readline error");
    logger::set_writer(stdout.clone());

    loop {

        let line = match rl.readline().await {
            Ok(ReadlineEvent::Line(line)) => {
                rl.add_history_entry(line.clone());
                line
            }
            Ok(ReadlineEvent::Eof) | Ok(ReadlineEvent::Interrupted) => {
                let _ = rl.flush();
                info("Closing MACNet...").await;
                exit(0);
            }
            Err(_) => {
                warn("Failed to read line, please open an issue on our girhub!").await;
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

                let players = sqlx::query("SELECT * FROM players ORDER BY id ASC")
                    .fetch_all(&STATE.get().unwrap().db)
                    .await;

                info(format!("Running for {}!", elapsed).as_str()).await;
                info(format!("Registered players: {}", players.unwrap().len()).as_str()).await;

            }
            "clear" => {

                writeln!(stdout, "\x1B[2J\x1B[1;1H").ok();

            }
            "exit" => {

                info("Closing MACNet...").await;
                let _ = rl.flush();
                exit(1);

            }
            _ => {}
        }

    }

    let _ = rl.flush();

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