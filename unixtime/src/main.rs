use chrono::Timelike;
use std::env;
use std::str::FromStr;

type NumberType = u32;

#[derive(Debug, Clone)]
enum ArgType {
    Exact(NumberType),
    Offset(NumberType),
}

impl FromStr for ArgType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_char = s.chars().next().ok_or(())?;

        let ret = if first_char == '+' {
            Self::Offset(s[1..].parse().map_err(|_| ())?)
        } else {
            Self::Exact(s.parse().map_err(|_| ())?)
        };

        Ok(ret)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args_iter = env::args().filter_map(|e| e.parse::<ArgType>().ok());

    let mut offset: Option<NumberType> = None;
    let mut time_parts = [0; 3];

    for (idx, arg) in args_iter.enumerate() {
        assert!(offset.is_none(), "got an arg after the offset");

        match arg {
            ArgType::Exact(t) => *time_parts.get_mut(idx).expect("too many args") = t,
            ArgType::Offset(t) => {
                offset.replace(t);
            }
        }
    }

    let mut now = chrono::offset::Local::now()
        .with_hour(time_parts[0])
        .expect("no hour")
        .with_minute(time_parts[1])
        .expect("no minute")
        .with_second(time_parts[2])
        .expect("no second");

    if let Some(offset) = offset {
        now += chrono::Duration::days(offset as i64);
    }

    let string: String = "<t:".to_string()
        + &now.timestamp().to_string()
        + ":R>\n<t:"
        + &(now.timestamp() + 1).to_string()
        + ":F>";

    println!("{}", string);

    Ok(())
}
