use crate::city::supply_kind::SupplyKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupplySpec {
    pub(crate) kind: SupplyKind,
    pub(crate) dead_line: usize,
    pub(crate) time_passed_ms: usize
}