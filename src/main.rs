use std::{
    collections::HashMap,
    env::args,
    process, thread,
    time::{Duration, Instant},
};

struct Config {
    minutes: u32,
    message: String,
    title: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            message: String::from("Take a break"),
            minutes: 25,
            title: String::from("Time left"),
        }
    }
}

fn duration_command(config: &mut Config, args: Option<&mut String>) {
    if let Some(t) = args {
        if let Ok(t) = t.parse::<u32>() {
            config.minutes = t
        }
    }
}

fn title_command(config: &mut Config, args: Option<&mut String>) {
    if let Some(t) = args {
        if let Ok(t) = t.parse() {
            config.title = t;
        }
    }
}

fn message_command(config: &mut Config, args: Option<&mut String>) {
    if let Some(t) = args {
        if let Ok(t) = t.parse() {
            config.message = t;
        }
    }
}

fn help_command(_config: &mut Config, _args: Option<&mut String>) {
    usage();
    process::exit(0);
}

fn main() {
    let mut config = Config::default();

    let mut args: Vec<String> = args().skip(1).collect();
    let mut args = args.iter_mut();
    let mut commands: HashMap<String, fn(config: &mut Config, args: Option<&mut String>)> =
        HashMap::new();
    commands.insert("-d".to_string(), duration_command);
    commands.insert("-h".to_string(), help_command);
    commands.insert("-t".to_string(), title_command);
    commands.insert("-m".to_string(), message_command);

    while let Some(arg) = args.next() {
        if let Some(command) = commands.get(arg.to_lowercase().as_str()) {
            command(&mut config, args.next())
        }
    }

    let total = Duration::new(u64::from(config.minutes) * 60, 0);
    let start = Instant::now();

    let handle = thread::spawn(move || 'inner: loop {
        let elapsed = start.elapsed();

        if elapsed >= total {
            break 'inner;
        }

        let remaining: u32 = match (total - elapsed).as_secs().try_into() {
            Ok(v) => v,
            Err(_) => {
                break 'inner;
            }
        };

        let hours = remaining / 60 / 60;
        let minutes = remaining / 60 - (hours * 60);
        let seconds = remaining - minutes * 60 - hours * 60 * 60;

        println!(
            "\x1B[2J\x1B[1;1H{}: {:0>2}:{:0>2}",
            config.title, minutes, seconds
        );

        thread::sleep(Duration::from_secs(1));
    });

    handle.join().unwrap();
    println!("{}", config.message);
}

fn usage() {
    println!("Usage: pomodoro [OPTION]... [ARGUMENT]...");
    println!("Basic pomodoro timer");
    println!("Options:");
    println!("  -d     duration in minutes (the default is 25)");
    println!("  -t     title (the default value is \"Time left\")");
    println!("  -m     a message that is displayed when the timer ends");
    println!("  -h     print this message");
}
