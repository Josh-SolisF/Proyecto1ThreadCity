use mypthreads::mythread::mythread::ThreadId;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryOutcome {
    GrantedFor { tid: ThreadId },   // Admisión concedida y recurso reservado
    Occupied,      // No ahora (rojo/ocupado); reintentar luego
}   