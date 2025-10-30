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

    pub fn ask_pass(&self) -> bool {
        !self.mutex.is_locked() && self.control.can_enter()
    }

    pub fn enter_bridge(&mut self) -> bool {
        if self.ask_pass() {
            self.mutex.lock(());
            true
        } else {
            false
        }
    }

    pub fn exit_bridge(&mut self) -> bool {
        self.mutex.unlock();
        true
    }

    pub fn open_bridge(&mut self) -> bool {
        // Barco solicita abrir el puente 3
        self.control.set_open(false);
        self.mutex.lock(());
        true
    }

    pub fn close_bridge(&mut self) -> bool {
        self.control.set_open(true);
        self.mutex.unlock(None);
        true
    }
    
}