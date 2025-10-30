use crate::cityblock::block_type::BlockType;
use crate::cityblock::coord::Coord;
use crate::cityblock::transport_policy::TransportPolicy;

pub struct BlockBase {
    pub(crate) id: usize,
    pub(crate) position: Coord,
    pub(crate) policy: TransportPolicy,
    pub(crate) occupied: bool,
    pub(crate) block_type: BlockType,
}

impl BlockBase {
    pub fn update(&mut self, delta_time_ms: usize) {
        // Cada tipo de bloque podría reaccionar distinto:
        match self.block_type {
            BlockType::Road => {} // nada especial
            BlockType::Bridge => {
                // en el futuro, podrías notificar al Bridge real que el tiempo pasó
            }
            BlockType::NuclearPlant => {
                // las plantas podrían actualizar su estado cada cierto tiempo
            }
            _ => {}
        }
    }
}
