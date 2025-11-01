mod control;
mod traffic_light;

use std::any::Any;
use mypthreads::mythread::mymutex::MyMutex;
use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::Block;
use crate::cityblock::block::BlockBase;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::bridge::control::Control;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::vehicle::vehicle::Vehicle;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Bridge {
    pub(crate) base: BlockBase,
    pub(crate) control: Control,
    pub(crate) mutex: MyMutex,
}

impl Block for Bridge {
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
}

impl Bridge {
    pub fn new(block: BlockBase, control: Control, mut mutex: MyMutex) -> Self {
        todo!()
    }

    pub fn ask_pass(&self, thread: &MyThread, vehicle_ty: VehicleType) -> bool {
        todo!()
    }

    pub fn enter_bridge(&mut self, vehicle: &Vehicle) -> bool {
        todo!()
    }

    pub fn exit_bridge(&mut self, vehicle: &Vehicle) -> bool {
        todo!()
    }

    pub fn open_bridge(&mut self, caller: &MyThread) -> bool {
        todo!()
    }

    pub fn close_bridge(&mut self, caller: &MyThread) -> bool {
        todo!()
    }
}