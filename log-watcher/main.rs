use clap::{Arg, ArgAction, Command};

fn main() {
    // Parse arguments
    let matches = Command::new("LogWatcher")
        .arg(Arg::new("log-file-path")
        .short('l')
        .required(true)
        .action(ArgAction::Set)
        .help("Path to log file"))
        .get_matches(); 
        
    let log_file_path = matches.get_one::<String>("log-file-path").unwrap();
    println!("Log file path: {}", log_file_path);
}
