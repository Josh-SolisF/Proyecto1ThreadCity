use crate::cityblock::block::BlockBase;

pub struct Map {
    pub(crate) grid: Vec<Vec<BlockBase>>,
}

impl Map {
    pub fn build_default() -> Map {
        todo!("Generar la ciudad a mano zzzz...")
    }
}