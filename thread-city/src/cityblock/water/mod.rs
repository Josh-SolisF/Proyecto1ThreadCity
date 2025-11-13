use std::any::Any;
use crate::cityblock::{Block, BlockBase};
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Water;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::Ship;

pub struct WaterBlock {
    pub(crate) base: BlockBase,
    has_space: bool,
}

impl Block for WaterBlock {
    fn get_id(&self) -> &usize {
        &self.base.id
    }
    fn get_policy(&self) -> &TransportPolicy {
        &self.base.policy
    }
    fn get_type(&self) -> &BlockType {
        &self.base.block_type
    }
    fn as_any(&mut self) -> &mut dyn Any  {
        self
    }
    fn can_pass(&self) -> bool {
        self.is_available()
    }
}

impl WaterBlock {
    pub fn new(id: usize) -> Self {
        Self {
            base: BlockBase::new(id, Ship, Water),
            has_space: true,
        }
    }
    pub fn consume_space(&mut self) -> bool {
        if !self.is_available() { return false }
        self.has_space = false;
        true
    }
    pub fn liberate_space(&mut self) {
        self.has_space = true;
    }
    #[inline]
    pub fn is_available(&self) -> bool { self.has_space }
}