use core::array;

use crate::config::MAX_SYSCALL_NUM;

#[derive(Copy, Clone)]
pub struct TaskInfo {
    pub id: usize,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
    pub start_time: usize,
}

#[derive(Copy, Clone)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize
}

impl TaskInfo {
    pub fn init() -> Self {
        Self {
            id: 0,
            call: array::from_fn(|i| SyscallInfo {
                id: i as usize,
                times: 0,
            }),
            time: 0,
            start_time: 0,
        }
    }
}