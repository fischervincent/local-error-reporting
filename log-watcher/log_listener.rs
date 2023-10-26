extern crate regex;
extern crate chrono;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::sync::mpsc::Sender;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use regex::Regex;
use chrono::Utc;

use crate::models::TraceOccurrence;

pub fn start(log_file_path: String, sender: Sender<TraceOccurrence>) {
    // Create a watcher object
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher.watch(Path::new(&log_file_path), RecursiveMode::Recursive).unwrap();


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

fn process_new_entries(log_file_path: &str, last_position: u64, sender: &Sender<TraceOccurrence>) -> io::Result<u64> {
    // Open the log file    
    let file = File::open(log_file_path)?;
    let mut file = BufReader::new(file);
    
    // Move to where we last stopped reading
    file.seek(SeekFrom::Start(last_position))?;

    let mut new_position = last_position;

    for line in file.lines() {
        let line = line?;
        new_position += line.as_bytes().len() as u64;

        // Here we create a TraceOccurrence for each new line. 
        // You might want to parse the line for timestamp and message details.
        let trace_occurrence = TraceOccurrence {
            timestamp: extract_timestamp_from_log_line(&line.to_string()),
            message: line,
        };

        sender.send(trace_occurrence).unwrap();
    }

    Ok(new_position)
}

fn extract_timestamp_from_log_line(line: &str) -> String {
    // This regex looks for sequences of digits between brackets that resemble a timestamp.
    let re = Regex::new(r"\[(\d{10,})\]").unwrap();
    
    // Capture the timestamp.
    if let Some(captures) = re.captures(line) {
        if let Some(matched) = captures.get(1) {
            return matched.as_str().to_string();
        }
    }
    
    // If we reach here, it means no timestamp was found.
    println!("Warning: No timestamp found in log. Using current time as the timestamp.");
    
    // Return the current UTC timestamp as a string.
    Utc::now().format("%s").to_string()
}
