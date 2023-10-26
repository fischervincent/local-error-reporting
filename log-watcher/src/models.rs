use std::fmt;

#[derive(Clone)]
pub struct Trace {
    pub first_seen: String, // Date time of the first occurrence
    pub last_seen: String,  // Date time of the last occurrence
    pub occurrences: Vec<TraceOccurrence>,
    pub count: u32,
}

#[derive(Clone)]
pub struct TraceOccurrence {
    pub datetime: String,
    pub message: String,
}

impl fmt::Display for TraceOccurrence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]: {}", self.datetime, self.message)
    }
}

pub struct UpdateEvent {
    pub trace_id: String,
    pub new_occurrence: TraceOccurrence,
    pub updated_trace: Trace,
}