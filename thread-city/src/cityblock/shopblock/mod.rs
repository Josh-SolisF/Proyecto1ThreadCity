use std::any::Any;
use crate::cityblock::{Block, BlockBase};
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::{Road, Shops};
use crate::cityblock::shopblock::shop::Shop;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::NoVehicles;

pub mod shop;

pub struct ShopBlock {
    pub(crate) base: BlockBase,
    pub(crate) shops: Vec<Shop>,
}
impl ShopBlock {
    pub(crate) fn new(id: usize, shops: Vec<Shop>) -> ShopBlock {
        Self {
            base: BlockBase::new(id, NoVehicles, Shops),
            shops,
        }
    }

    pub fn get_shops(&self) -> &Vec<Shop> {
        &self.shops
    }
}
impl Block for ShopBlock {
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
}