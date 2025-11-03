pub mod control;
mod traffic_light;
pub(crate) mod BridgePermissionEnum;

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
        if let Some(tl) = &self.control.in_traffic_light {
            !tl.can_pass() // rojo => bloqueado
        }

        else {false}
    }

fn as_any(&self) -> &dyn Any {self}


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

        // Política: ¿este vehículo puede usar el puente?
        if !self.base.policy.can_pass(vehicle_ty) {
            return false;
        }
        // Semáforo de entrada: si existe, debe estar en verde
        if let Some(tl) = &self.control.in_traffic_light {
            return tl.can_pass();
        }
        // Sin semáforo = libre
        true

    }

    pub(crate) fn request_entry(&mut self, v: &VehicleBase, _from: Coord, _to: Coord, caller:ThreadId) -> EntryOutcome {
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


    pub fn advance_time(&mut self, step_ms: usize) {
        self.control.advance_time(step_ms);
    }
    

    pub fn enter_bridge(&mut self, vehicle: &VehicleBase, caller:ThreadId) -> bool {


        // Política
        if !self.base.policy.can_pass(vehicle.vehicle_type) {
            return false;
        }
        // Semáforo de entrada
        if let Some(tl) = &self.control.in_traffic_light {
            if !tl.can_pass() {
                return false;
            }
        }

        // Intentar reservar el único carril
        match self.mutex.as_mut() {
            Some(m) => {
                let rc = m.try_lock(caller);
                rc == 0
            }
            None => false, // no debería ocurrir: P1 siempre tiene mutex
        }

    }


    pub fn exit_bridge(&mut self, _vehicle: &VehicleBase, caller: ThreadId) -> bool {
        match self.mutex.as_mut() {
            Some(m) => {
                // Asumiendo API unlock(ThreadId) -> i32
                // Ajusta el nombre si tu MyMutex usa otra firma
                let rc = m.unlock(caller);
                rc == 0
            }
            None => false,
        }
    }


    pub fn open_bridge(&mut self, _caller: &MyThread) -> bool {
        if let Some(tl) = &mut self.control.in_traffic_light {
            tl.force_red();
        }
        if let Some(tl) = &mut self.control.out_traffic_light {
            tl.force_red();
        }
        true
    }


    pub fn close_bridge(&mut self, _caller: &MyThread) -> bool {
        if let Some(tl) = &mut self.control.in_traffic_light {
            tl.force_green();
        }
        if let Some(tl) = &mut self.control.out_traffic_light {
            tl.force_green();
        }
        true
    }

    pub fn return_mutex(&mut self) -> Option<MyMutex> {
        self.mutex.take()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}