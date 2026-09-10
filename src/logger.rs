use chrono::{ Local };

enum Type {
    INFO,
    WARN,
    ERROR
}

pub async fn info(message_str: &str) {

    let message = message_str.to_string();
    let _ = log_message(message, Type::INFO).await;

}

pub async fn warn(message_str: &str) {

    let message = message_str.to_string();
    let _ = log_message(message, Type::WARN).await;

}

pub async fn error(message_str: &str) {

    let message = message_str.to_string();
    let _ = log_message(message, Type::ERROR).await;

}

async fn log_message(message: String, log_type: Type) {

    // Logging to console

    let now = Local::now();
    let time = now.format("%Y:%m:%d %H:%M:%S").to_string();
    let color = get_color(&log_type);

    println!("{}[{}] {}\x1b[0m", color, time, message);

}

fn get_color(log_type: &Type) -> String {

    let value;

    match log_type {
        Type::INFO => value = "\x1b[0m",
        Type::WARN => value = "\x1b[33m",
        Type::ERROR => value = "\x1b[31m",
    }

    value.to_string()

}

fn get_type(log_type: &Type) -> String {

    match log_type {
        Type::INFO => String::from("INFO"),
        Type::WARN => String::from("WARN"),
        Type::ERROR => String::from("ERROR"),
    }

}