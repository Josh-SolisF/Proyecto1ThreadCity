/*
use libc::{
    pthread_attr_t,
    pthread_attr_init,
    pthread_attr_destroy,
    pthread_attr_setdetachstate,
    pthread_attr_setstacksize,
    PTHREAD_CREATE_DETACHED,
    PTHREAD_CREATE_JOINABLE,
};
*/

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MyAttr {
    pub detached: bool,
    pub stack_size: usize,
}

impl Default for MyAttr {
    fn default() -> Self {
        Self { detached: false, stack_size: 0 }
    }
}

#[repr(transparent)]
pub struct MyThreadAttr {
    inner: MyAttr,
}


impl MyThreadAttr {
    pub fn new() -> Self {
        Self { inner: MyAttr::default() }
    }
    pub fn set_detached(&mut self, detached: bool) { self.inner.detached = detached; }
    pub fn set_stack_size(&mut self, size: usize) { self.inner.stack_size = size; }

    /// Para mypthread que esperan puntero (p. ej. my_thread_create acepta *const MyAttr).
    pub fn as_ptr(&self) -> *const MyAttr { &self.inner }

    /// Para copiar el valor dentro de create().
    pub fn into_value(self) -> MyAttr { self.inner }
}

/*
    /// Configura el tamaño de la pila
    pub fn set_stack_size(&mut self, size: usize) {
        unsafe {
            pthread_attr_setstacksize(&mut self.inner, size);
        }
    }

    /// Devuelve un puntero al pthread_attr_t interno (para pasar a pthread_create)
    pub fn as_ptr(&self) -> *const MyAttr {
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
*/