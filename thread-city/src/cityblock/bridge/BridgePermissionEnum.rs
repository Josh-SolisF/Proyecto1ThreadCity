pub enum EntryOutcome {
    Granted,   // Admisión concedida y recurso reservado
    Wait,      // No ahora (rojo/ocupado); reintentar luego
    Forbidden, // Nunca (política prohíbe)
}   