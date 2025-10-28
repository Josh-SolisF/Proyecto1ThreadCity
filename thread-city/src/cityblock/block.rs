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