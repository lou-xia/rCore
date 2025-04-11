use crate::config::PAGE_SIZE;
use crate::mm::{translated_byte_timeval, MapPermission};
use crate::task::{current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let now_time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    // println!("now time: {:?}", now_time);
    translated_byte_timeval(current_user_token(), ts as usize, now_time);
    // println!("ts time: {:?}", unsafe {
    //     ts
    // });
    0
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    if prot == 0 || prot & !7 != 0 {
        return -1;
    }
    let mut perm: MapPermission = MapPermission::U;
    if prot & 1 != 0 {perm.insert(MapPermission::R);}
    if prot & 2 != 0 {perm.insert(MapPermission::W);}
    if prot & 4 != 0 {perm.insert(MapPermission::X);}
    TASK_MANAGER.current_task_mmap(start, len, perm)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    TASK_MANAGER.current_task_munmap(start, len)
}