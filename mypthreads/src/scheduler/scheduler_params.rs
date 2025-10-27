//! De momento los usa solo RR/FIFO a modo informativo

use super::trait::ThreadId;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct SchedulerParams {
    /// Time-slice recomendado en milisegundos .
    pub timeslice_ms: Option<u64>,

    /// Prioridades por hilo .
    pub priorities: Option<HashMap<ThreadId, u8>>,

    /// Tickets por hilo.
    pub tickets: Option<HashMap<ThreadId, u32>>,
}

impl SchedulerParams {
    pub fn with_timeslice(ms: u64) -> Self {
        Self { timeslice_ms: Some(ms), ..Default::default() }
    }
}