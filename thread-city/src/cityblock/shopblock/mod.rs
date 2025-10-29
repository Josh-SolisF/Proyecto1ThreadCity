use crate::cityblock::block::BlockBase;
use crate::cityblock::shopblock::shop::Shop;

mod shop;

pub struct ShopBlock {
    pub(crate) block: BlockBase,
    pub(crate) shops: Vec<Shop>,
}