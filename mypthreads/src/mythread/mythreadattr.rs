use libc::{
    pthread_attr_t,
    pthread_attr_init,
    pthread_attr_destroy,
    pthread_attr_setdetachstate,
    pthread_attr_setstacksize,
    PTHREAD_CREATE_DETACHED,
    PTHREAD_CREATE_JOINABLE,
};

pub type myAttr = pthread_attr_t;

#[repr(transparent)]
pub struct MyThreadAttr {
    inner: myAttr,
}

impl MyThreadAttr {
    pub fn new() -> Self {
        unsafe {
            let mut attr: myAttr = std::mem::zeroed();
            pthread_attr_init(&mut attr);
            Self { inner: attr }
        }
    }

    /// Configura el modo detached o joinable
    pub fn set_detached(&mut self, detached: bool) {
        let state = if detached { PTHREAD_CREATE_DETACHED } else { PTHREAD_CREATE_JOINABLE };
        unsafe {
            pthread_attr_setdetachstate(&mut self.inner,state);
        }
    }

    /// Configura el tamaño de la pila
    pub fn set_stack_size(&mut self, size: usize) {
        unsafe {
            pthread_attr_setstacksize(&mut self.inner, size);
        }
    }

    /// Devuelve un puntero al pthread_attr_t interno (para pasar a pthread_create)
    pub fn as_ptr(&self) -> *const myAttr {
        &self.inner
    }
}

impl Drop for MyThreadAttr {
    fn drop(&mut self) {
        unsafe {
            pthread_attr_destroy(&mut self.inner);
        }
    }
}
