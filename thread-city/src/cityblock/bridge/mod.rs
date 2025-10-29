use mypthreads::mythread::mymutex::MyMutex;
use crate::cityblock::block::BlockBase;
use crate::cityblock::bridge::control::Control;

mod control;

pub struct Bridge {
    pub(crate) block: BlockBase,
    pub(crate) control: Control,
    pub(crate) mutex: MyMutex,
}

impl Bridge {
    pub fn new(block: BlockBase, control: Control, mutex: MyMutex) -> Self {
        Self {
            block,
            control,
            mutex,
        }
    }
    
    /// Consulta el estado de locked del mutex del puente
    pub fn ask_pass() -> bool {
        todo!()
    } 
    /// Hace lock al mutex dándole el recurso al vehículo que lo haya pedido
    pub fn enter_bridge() -> bool {
        todo!()
    }
    /// Hace unlock al mutex.
    pub fn exit_bridge() -> bool {
        todo!()
    }
    /// Hace lock al mutex dándole el recurso al barco que lo haya pedido
    pub fn open_bridge() -> bool {
        todo!()
    }
    /// Hace unlock al mutex.
    pub fn close_bridge() -> bool {
        todo!()
    }
    
}