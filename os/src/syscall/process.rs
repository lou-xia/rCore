
use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    get_task_info
};
use crate::timer::get_time_us;
use crate::task::TaskInfo;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    let task_id = crate::task::get_current_task_id();
    println!("[kernel] Application {} exited with code {}", task_id, exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// pub fn sys_get_time() -> isize {
//     get_time_ms() as isize
// }

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

pub fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    if let Some(mut tcb) = get_task_info(id) {
        let current_time = get_time_us();
        tcb.task_info.time += current_time - tcb.task_info.start_time;
        unsafe {
            *ts = tcb.task_info;
        }
    } else {
        return -1;
    }
    0
}