pub struct MyMutexAttr {
    pub kind: i32,
}

impl MyMutexAttr {
    pub fn new(kind: i32) -> MyMutexAttr {
        MyMutexAttr {
            kind
        }
    }
}