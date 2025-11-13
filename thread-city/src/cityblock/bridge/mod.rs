pub mod control;
mod traffic_light;
pub mod bridge_permision_enum;

use std::any::Any;
use mypthreads::mythread::mymutex::MyMutex;
use mypthreads::mythread::mypthread::MyPThread;
use crate::cityblock::Block;
use crate::cityblock::block::BlockBase;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Bridge;
use crate::cityblock::bridge::bridge_permision_enum::EntryOutcome;
use crate::cityblock::bridge::bridge_permision_enum::EntryOutcome::{GrantedFor, Occupied};
use crate::cityblock::bridge::control::Control;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::{AnyVehicle, Car};
use crate::vehicle::vehicle::{PatienceLevel, Vehicle, VehicleBase};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::ShipE;

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
    fn as_any(&mut self) -> &mut dyn Any {self}
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
    pub fn request_entry(&mut self, vehicles: Vec<&Box<dyn Vehicle>>) -> EntryOutcome {
        let candidate = self.control.allow_in(vehicles);
        if candidate.is_none() { return Occupied }
        match self.mutex.as_mut() {
            Some(m) => {
                let rc: i32 = m.try_lock(candidate.unwrap());
                if rc == 0 {
                    GrantedFor { tid: candidate.unwrap() } // carril reservado
                } else {
                    Occupied
                }
            }
            None => Occupied,
        }
    }
    pub fn exit_bridge(&mut self, v_type: VehicleType, v_pat: PatienceLevel) -> bool {
        let mut pth = MyPThread::new();
        let allowed = (v_type == ShipE) || self.control.allow_out(v_type, v_pat);

        if allowed {
            match self.mutex.as_mut() {
                Some(m) => {
                    unsafe { pth.my_mutex_unlock(m); }
                }
                None => {
                }
            }
            return true;
        }
        false
    }


    pub fn return_mutex(&mut self) -> Option<MyMutex> {
        self.mutex.take()
    }
    pub fn advance_time(&mut self, frames: usize) {
        self.control.advance_time(frames);
    }
}