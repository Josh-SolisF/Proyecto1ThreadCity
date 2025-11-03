use std::any::Any;
use crate::cityblock::{Block, BlockBase};
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Dock;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::Ship;

pub struct DockBlock {
    pub(crate) base: BlockBase,
}

impl Block for DockBlock {
    fn get_id(&self) -> &usize {
        &self.base.id
    }

    fn get_policy(&self) -> &TransportPolicy {
        &self.base.policy
    }

    fn get_type(&self) -> &BlockType {
        &self.base.block_type
    }

    fn is_blocked(&self) -> bool {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl DockBlock {
    pub fn new(id: usize) -> Self {
        Self {
            base: BlockBase::new(id, Ship, Dock),
        }
    }
}