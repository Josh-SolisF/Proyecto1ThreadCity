pub mod control;
mod traffic_light;

use std::any::Any;
use mypthreads::mythread::mymutex::MyMutex;
use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::Block;
use crate::cityblock::block::BlockBase;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Bridge;
use crate::cityblock::bridge::control::Control;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::{AnyVehicle, Car};
use crate::vehicle::vehicle::VehicleBase;
use crate::vehicle::vehicle_type::VehicleType;

pub struct BridgeBlock {
    pub(crate) base: BlockBase,
    pub(crate) control: Control,
    pub(crate) mutex: Option<MyMutex>,
}

impl Block for BridgeBlock {
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

impl BridgeBlock {
    pub fn new(id: usize, control: Control, bridge_mutex: MyMutex) -> Self {
        let policy : TransportPolicy = if control.can_pass_boats { AnyVehicle } else { Car };
        Self {
            base: BlockBase::new(id, policy, Bridge),
            control,
            mutex: Some(bridge_mutex),
        }
    }

    pub fn ask_pass(&self, thread: &MyThread, vehicle_ty: VehicleType) -> bool {
        todo!()
    }

    pub fn enter_bridge(&mut self, vehicle: &VehicleBase) -> bool {
        todo!()
    }

    pub fn exit_bridge(&mut self, vehicle: &VehicleBase) -> bool {
        todo!()
    }

    pub fn open_bridge(&mut self, caller: &MyThread) -> bool {
        todo!()
    }

    pub fn close_bridge(&mut self, caller: &MyThread) -> bool {
        todo!()
    }

    pub fn return_mutex(&mut self) -> Option<MyMutex> {
        self.mutex.take()
    }
}