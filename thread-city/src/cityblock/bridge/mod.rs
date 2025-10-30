mod control;
mod bridge_type;
mod test;

use std::os::raw::c_int;
use mypthreads::mythread::mymutex::MyMutex;
use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::block::BlockBase;
use crate::cityblock::bridge::control::Control;
use crate::vehicle::vehicle::Vehicle;
use crate::vehicle::vehicle_type::VehicleType;

// Si puedes, importa los códigos para comparar explícitamente:
use mypthreads::mythread::mypthreadexits::Exits::{
    Ok as EX_OK,
    MutexLockApproved,
    MutexLocked,
    MutexNotInitialized,
    MutexInvalidOwner,
};

pub struct Bridge {
    pub(crate) block: BlockBase,
    pub(crate) control: Control,
    pub(crate) mutex: MyMutex,
}

impl Bridge {
    pub fn new(block: BlockBase, control: Control, mut mutex: MyMutex) -> Self {
        // Asegura que el mutex esté inicializado
        // (puedes mover esto afuera si prefieres inicializarlo al crearlo)
        #[allow(unused_unsafe)]
        unsafe { let _ = mutex.init_mut(); }
        // o: mutex.init();

        Self { block, control, mutex }
    }

    /// Evalúa reglas de control (política, semáforo, abierto) y, opcionalmente,
    /// el estado del lock (si implementaste is_locked()).
    pub fn ask_pass(&self, thread: &MyThread, vehicle_ty: VehicleType) -> bool {
        // Si no quieres depender de is_locked() aquí, comenta la siguiente línea:
        // let free = !self.mutex.is_locked();
        // free && self.control.can_enter(thread, vehicle_ty)

        // Versión que solo valida las reglas del Control
        self.control.can_enter(thread, vehicle_ty)
    }

    /// Intenta entrar: valida reglas y trata de tomar el lock.
    /// Usa try_lock para que sea no bloqueante en tests.
    pub fn enter_bridge(&mut self, vehicle: &Vehicle) -> bool {
        if !self.control.can_enter(vehicle.thread(), vehicle.ty()) {
            return false;
        }
        let tid = vehicle.thread().id();
        let rc: c_int = self.mutex.try_lock(tid);

        // Éxito si retorna Ok (try_lock) o (por compatibilidad) MutexLockApproved
        rc == (EX_OK as c_int) || rc == (MutexLockApproved as c_int)
    }

    /// Debe desbloquear el mismo tid que bloqueó, si no -> MutexInvalidOwner
    pub fn exit_bridge(&mut self, vehicle: &Vehicle) -> bool {
        let tid = Some(vehicle.thread().id());
        let rc: c_int = self.mutex.unlock(tid);
        rc == (EX_OK as c_int)
    }

    /// Abrir puente (para barcos): necesita un "caller" para ser el owner del lock.
    /// Sugerencia: que el hilo que solicita apertura sea quien llame aquí.
    pub fn open_bridge(&mut self, caller: &MyThread) -> bool {
        self.control.set_open(false);
        let rc: c_int = self.mutex.lock(caller.id());
        rc == (MutexLockApproved as c_int) || rc == (EX_OK as c_int)
    }

    pub fn close_bridge(&mut self, caller: &MyThread) -> bool {
        self.control.set_open(true);
        let rc: c_int = self.mutex.unlock(Some(caller.id()));
        rc == (EX_OK as c_int)
    }
}