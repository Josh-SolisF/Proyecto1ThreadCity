use std::any::Any;
use crate::cityblock::{Block, BlockBase};
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Road;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::Car;

pub struct RoadBlock {
    pub(crate) base: BlockBase,
    pub(crate) space: u8,
}

impl Block for RoadBlock {
    fn get_id(&self) -> &usize {
        &self.base.id
    }
    fn get_policy(&self) -> &TransportPolicy {
        &self.base.policy
    }
    fn get_type(&self) -> &BlockType {
        &self.base.block_type
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn can_pass(&self) -> bool {
        self.is_available()
    }
}

impl RoadBlock {
    pub fn new(id: usize) -> RoadBlock {
        Self {
            base: BlockBase::new(id, Car, Road),
            space: 4,
        }
    }
    #[inline]
    pub fn is_available(&self) -> bool { self.space > 0 }
    pub fn consume_space(&mut self) -> bool {
        if !self.is_available() { return false }
        self.space -= 1;
        true
    }
    pub fn liberate_space(&mut self) {
        self.space += 1;
    }
}