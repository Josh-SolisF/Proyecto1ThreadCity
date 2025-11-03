use std::any::Any;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::transport_policy::TransportPolicy;
pub struct BlockBase {
    pub(crate) id: usize,
    pub(crate) policy: TransportPolicy,
    pub(crate) block_type: BlockType,
}

impl BlockBase {
    pub fn new(
        id: usize,
        policy: TransportPolicy,
        block_type: BlockType,
    ) -> Self {
        Self {
            id,
            policy,
            block_type,
        }
    }
}


pub trait Block: Any {
    fn get_id(&self) -> &usize;
    fn get_policy(&self) -> &TransportPolicy;
    fn get_type(&self) -> &BlockType;
    fn is_blocked(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}