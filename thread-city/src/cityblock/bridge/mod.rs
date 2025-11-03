pub mod control;
mod traffic_light;
mod BridgePermissionEnum;

use std::any::Any;
use mypthreads::mythread::mymutex::MyMutex;
use mypthreads::mythread::mythread::{MyThread, ThreadId};
use crate::cityblock::Block;
use crate::cityblock::block::BlockBase;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Bridge;
use crate::cityblock::bridge::BridgePermissionEnum::EntryOutcome;
use crate::cityblock::bridge::control::Control;
use crate::cityblock::coord::Coord;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::{AnyVehicle, Car};
use crate::vehicle::vehicle::{Vehicle, VehicleBase};
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

    /// Puente 1: un carril + semáforo global (intervalo en ms)
    pub fn new_p1(id: usize, interval_ms: usize, bridge_mutex: MyMutex) -> Self {
        let control = Control::with_traffic(interval_ms, interval_ms);
        let policy = TransportPolicy::Car;
        Self {
            base: BlockBase::new(id, policy, BlockType::Bridge),
            control,
            mutex: Some(bridge_mutex),
        }
    }


    pub fn ask_pass(&self, thread: &MyThread, vehicle_ty: VehicleType) -> bool {
        todo!()
    }

    fn request_entry(&mut self, v: &VehicleBase, _from: Coord, _to: Coord, caller:ThreadId) -> EntryOutcome {
        // Política puede usar el puente (para que los barcos no pasen por tierra)
        if !self.base.policy.can_pass(v.vehicle_type) {
            return EntryOutcome::Forbidden;
        }
        // Puente 1: semáforo verde requerido
        if let Some(tl) = &self.control.in_traffic_light {
            if !tl.can_pass() {return EntryOutcome::Wait;}
        }

        match self.mutex.as_mut() {
            Some(m) => {

                let rc: i32 = m.try_lock(caller);
                if rc == 0 {
                    EntryOutcome::Granted // carril reservado


                } else {
                    EntryOutcome::Wait // ocupado, reintenta luego
                }
            }
            None => EntryOutcome::Wait, // sin mutex configurado
        }
    }

    pub fn enter_bridge(&mut self, vehicle: &VehicleBase) -> bool {
        todo!()
    }

    pub fn exit_bridge(&mut self, vehicle: &Box<dyn Vehicle>) -> bool {
        todo!()
    }   

    pub fn return_mutex(&mut self) -> Option<MyMutex> {
        self.mutex.take()
    }
}