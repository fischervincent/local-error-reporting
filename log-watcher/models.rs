use std::fmt;

#[derive(Clone)]
pub struct Trace {
    pub first_seen: String, // Timestamp of the first occurrence
    pub last_seen: String,  // Timestamp of the last occurrence
    pub occurrences: Vec<TraceOccurrence>,
    pub count: u32,
}

#[derive(Clone)]
pub struct TraceOccurrence {
    pub timestamp: String,
    pub message: String,
}

impl fmt::Display for TraceOccurrence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]: {}", self.timestamp, self.message)
    }
}

pub struct UpdateEvent {
    pub trace_id: String,
    pub new_occurrence: TraceOccurrence,
    pub updated_trace: Trace,
}