# Local Log Watcher (WIP)
## Overview

When debugging locally, sorting through raw logs can be tedious.

Local Log Watcher streamlines this by monitoring a local log file, intelligently grouping similar entries, and presenting updates in real-time via WebSocket. This makes it easier to spot patterns and anomalies during local testing and development.

## Features
- **Real-time Monitoring**: Listens to changes in the specified log file and processes new entries on-the-fly.
- **Log Grouping**: Identifies similar log entries and groups them into "Traces".
- **WebSocket Server**: Provides a WebSocket endpoint that pushes updates to connected clients in real-time.
- **Detailed Trace Analytics**: Captures the first occurrence, last occurrence, total occurrences, and more for each trace.

## Usage
```
./log-watcher --log-file-path /path/to/your/logfile.log
```

### Command-Line Arguments
**--log-file-path**: Specifies the path to the log file you wish to monitor.


## Setup for dev
### Prerequisites
- Rust 1.50 or newer
- Cargo

