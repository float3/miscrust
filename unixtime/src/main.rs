use arboard::Clipboard;
use chrono::Timelike;
use std::env;
fn main() {
    let env: Vec<u32> = env::args().filter_map(|x| x.parse().ok()).collect();
    let mut clipboard = Clipboard::new().unwrap();

    let now = chrono::offset::Local::now()
        .with_hour(env[0])
        .unwrap()
        .with_minute(env[1])
        .unwrap()
        .with_second(env[2])
        .unwrap();

    let string: String = "<t:".to_string() + &now.timestamp().to_string() + ":R>";

    let _ = clipboard.set_text(string);
}
