use std::sync::atomic::AtomicBool;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::coord::Coord;
use crate::cityblock::transport_policy::TransportPolicy;

pub struct BlockBase {
    pub(crate) id: usize,
    pub(crate) position: Coord,
    pub(crate) policy: TransportPolicy,
    pub(crate) occupied: AtomicBool,
    pub(crate) block_type: BlockType,
}

impl BlockBase {


    pub fn new(
        id: usize,
        position: Coord,
        policy: TransportPolicy,
        block_type: BlockType,
    ) -> Self {
        Self {
            id,
            position,
            policy,
            occupied: AtomicBool::new(false),
            block_type,
        }
    }


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

impl Default for BlockBase {
    fn default() -> Self {
        Self::new(
            0,
            Coord::new(0, 0),
            TransportPolicy::Any,
            BlockType::Road,
        )
    }
}

