use clap::{Arg, ArgAction, Command};
use std::panic;
use std::sync::mpsc::channel;

mod log_listener;
mod models;

fn main() {
    // Parse arguments
    let matches = Command::new("LogWatcher")
        .arg(
            Arg::new("log-file-path")
                .short('l')
                .required(true)
                .action(ArgAction::Set)
                .help("Path to log file"),
        )
        .get_matches();

    let log_file_path = matches
        .get_one::<String>("log-file-path")
        .unwrap()
        .to_string();
    println!("Log file path: {}", log_file_path);

    let (trace_sender, trace_receiver) = channel();

    std::thread::spawn(move || {
        let result = panic::catch_unwind(|| {
            log_listener::start(log_file_path.clone(), trace_sender);
        });

        if let Err(err) = result {
            println!("Error in log_listener thread: {:?}", err);
        }
    });

    while let Ok(trace_occurrence) = trace_receiver.recv() {
        println!("Received log line: {}", trace_occurrence);
    }

    if let Err(err) = trace_receiver.recv() {
        println!("Error receiving from channel: {:?}", err);
    }
}
