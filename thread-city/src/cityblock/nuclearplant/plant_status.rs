
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlantStatus {
    Ok,
    AtRisk,
    Critical,
    Boom,
}