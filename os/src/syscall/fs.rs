//! File and filesystem-related syscalls

const FD_STDOUT: usize = 1;

use crate::batch::APP_MANAGER;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !safe_check(buf) {
        return -1;
        // panic!("Invalid buf pointer in sys_write, buf: {:?}", buf);
    }

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            return -1;
            // panic!("Unsupported fd in sys_write!");
        }
    }
}

fn safe_check(buf: *const u8) -> bool {
    let app_start = APP_MANAGER.exclusive_access().get_app_start();
    let current_app = APP_MANAGER.exclusive_access().get_current_app();
    let num_app = APP_MANAGER.exclusive_access().get_num_app();

    // println!("safe_check, buf: {:?}, app_start: {:?}, current_app: {}, num_app: {}", buf, app_start, current_app, num_app);

    if buf < app_start[num_app] as *const u8 {
        if buf < app_start[current_app + 1] as *const u8 && buf >= app_start[current_app] as *const u8 {
            true
        } else {
            false
        }
    } else {
        true
    }
}
