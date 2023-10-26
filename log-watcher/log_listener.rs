extern crate chrono;
extern crate regex;

use chrono::Utc;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use crate::models::TraceOccurrence;

pub fn start(log_file_path: String, sender: Sender<TraceOccurrence>) {
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(Path::new(&log_file_path), RecursiveMode::Recursive)
        .unwrap();

    // Keep track of where we last stopped reading in the file
    let mut last_position: u64 = 0;

    loop {
        match rx.recv() {
            Ok(_) => {
                last_position = process_new_entries(&log_file_path, last_position, &sender)
                    .unwrap_or(last_position);
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

fn process_new_entries(
    log_file_path: &str,
    last_position: u64,
    sender: &Sender<TraceOccurrence>,
) -> io::Result<u64> {
    // Open the log file
    let file = File::open(log_file_path)?;
    let mut file = BufReader::new(file);

    // Move to where we last stopped reading
    file.seek(SeekFrom::Start(last_position))?;

    let mut new_position = last_position;

    for line in file.lines() {
        let line = line?;
        new_position += line.as_bytes().len() as u64;

        match extract_timestamp_from_log_line(&line.to_string()) {
            Ok(timestamp) => {
                let trace_occurrence = TraceOccurrence {
                    timestamp: timestamp,
                    message: line,
                };
                sender.send(trace_occurrence).unwrap();
            }
            Err(e) => {
                // it happens that the file is not flushed yet, so we can't extract the timestamp
                // from the line. We'll just ignore the line and try again next time.
                println!(
                    "Failed to extract timestamp: {:?} from line {:?}, line ignored.",
                    e,
                    &line.to_string()
                );
                continue;
            }
        }
    }

    Ok(new_position)
}

#[derive(Debug)]
pub enum LogError {
    TimestampNotFound,
}

fn extract_timestamp_from_log_line(line: &str) -> Result<String, LogError> {
    let re = Regex::new(r"\[(\d+)]").unwrap();
    if let Some(captures) = re.captures(line) {
        captures
            .get(1)
            .map(|m| Ok(m.as_str().to_string()))
            .unwrap_or(Err(LogError::TimestampNotFound))
    } else {
        Err(LogError::TimestampNotFound)
    }
}
