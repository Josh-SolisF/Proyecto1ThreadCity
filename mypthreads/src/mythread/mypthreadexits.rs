pub enum Exits {
    Ok = 0,
    ThreadBlocked = 1,
    MutexNotInitialized = 2,
    MutexInvalidState = 3,
    NullMutex = 4,
    MutexLockApproved = 5,
    MutexLocked = 6,
    CurrentIsEmpty = 7,
    ThreadIsTerminated = 8,
    UnknownThread = 9,
    MutexInvalidOwner = 10,
}